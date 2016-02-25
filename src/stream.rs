use document::DocumentHandle;
use haru;
use std::{mem, slice};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::DerefMut;
use std::os::raw::c_void;

/// Converts a Rust `Read + Seek` type instance into a stream object consumable by libharu.
///
/// # Safety
///
/// The returned `haru::HPDF_Stream` **must** have its ownership consumed to avoid memory leaks.
/// Consumtion is either done by passing the stream to a libharu function that frees it, or by
/// manually using `haru::HPDF_Stream_Free`.
pub unsafe fn convert_read_stream<R: Read + Seek>(handle: &DocumentHandle,
                                                  r: R) -> haru::HPDF_Stream {
    let mut reader = Box::new(TellingReader { reader: r, pos: 0 });
    let stream = haru::HPDF_CallbackReader_New(
        haru::HPDF_GetMMgr(handle.0),
        Some(read::<R>),
        Some(seek::<R>),
        Some(tell::<R>),
        Some(size::<R>),
        Some(free::<R>),
        reader.deref_mut() as *mut TellingReader<R> as *mut c_void
    );
    mem::forget(reader);
    stream
}

/// Converts a Rust `Write` type instance into a stream object consumable by libharu.
///
/// # Safety
///
/// The returned `haru::HPDF_Stream` **must** have its ownership consumed to avoid memory leaks.
/// Consumtion is either done by passing the stream to a libharu function that frees it, or by
/// manually using `haru::HPDF_Stream_Free`.
pub unsafe fn convert_write_stream<W: Write>(handle: &DocumentHandle,
                                             w: &mut W) -> haru::HPDF_Stream {
    let mmgr = haru::HPDF_GetMMgr(handle.0);
    let w = w as *mut W as *mut c_void;
    haru::HPDF_CallbackWriter_New(mmgr, Some(write::<W>), w)
}

unsafe extern fn read<R: Read + Seek>(stream: haru::HPDF_Stream, ptr: *mut haru::HPDF_BYTE,
                                      size: *mut haru::HPDF_UINT) -> haru::HPDF_STATUS {
    let r: &mut TellingReader<R> = mem::transmute((*stream).attr);
    let buf: &mut [u8] = slice::from_raw_parts_mut(ptr, *size as usize);

    let mut read_total = 0;
    while read_total < buf.len() {
        let read_len = match r.reader.read(&mut buf[read_total..]) {
            Ok(0) => {
                *size = read_total as haru::HPDF_UINT;
                // EOF
                return 0x1058;
            }
            Ok(read_len) => read_len,
            Err(err) => {
                *size = 0;
                let detail = err.raw_os_error().unwrap_or(0) as u64;
                return haru::HPDF_RaiseError((*stream).error, 0x1016, detail);
            },
        };

        read_total += read_len;
    }

    r.pos += read_total as u64;
    0
}

extern fn seek<R: Read + Seek>(stream: haru::HPDF_Stream, pos: haru::HPDF_INT,
                               mode: haru::HPDF_WhenceMode) -> haru::HPDF_STATUS {
    use haru::Enum__HPDF_WhenceMode::*;

    let r: &mut TellingReader<R> = unsafe { mem::transmute((*stream).attr) };

    let result = r.reader.seek(match mode {
        HPDF_SEEK_CUR => SeekFrom::Current(pos as i64),
        HPDF_SEEK_END => SeekFrom::End(pos as i64),
        HPDF_SEEK_SET => SeekFrom::Start(pos as u64),
    });

    match result {
        Ok(sought_to) => {
            r.pos = sought_to;
            0
        },
        Err(err) => {
            let detail = err.raw_os_error().unwrap_or(0) as u64;
            unsafe { haru::HPDF_RaiseError((*stream).error, 0x1016, detail) }
        },
    }
}

extern fn tell<R: Read + Seek>(stream: haru::HPDF_Stream) -> haru::HPDF_INT32 {
    let r: &mut TellingReader<R> = unsafe { mem::transmute((*stream).attr) };
    r.pos as haru::HPDF_INT32
}

extern fn size<R: Read + Seek>(stream: haru::HPDF_Stream) -> haru::HPDF_UINT32 {
    let r: &mut TellingReader<R> = unsafe { mem::transmute((*stream).attr) };
    let saved_pos = r.pos;
    let ret = r.reader.seek(SeekFrom::End(0)).ok().unwrap_or(0);
    if let Err(err) = r.reader.seek(SeekFrom::Start(saved_pos)) {
        let detail = err.raw_os_error().unwrap_or(0) as u64;
        unsafe { haru::HPDF_SetError((*stream).error, 0x1016, detail); }
        0
    } else {
        ret as haru::HPDF_UINT32
    }
}

extern fn free<R: Read + Seek>(stream: haru::HPDF_Stream) {
    let r: Box<TellingReader<R>> = unsafe { mem::transmute((*stream).attr) };
    drop(r)
}

struct TellingReader<R: Read + Seek> {
    pub reader: R,
    pub pos: u64
}

unsafe extern fn write<W: Write>(stream: haru::HPDF_Stream, ptr: *const haru::HPDF_BYTE,
                                 size: haru::HPDF_UINT) -> haru::HPDF_STATUS {
    let w: &mut W = mem::transmute((*stream).attr);
    let buf: &[u8] = slice::from_raw_parts(ptr, size as usize);
    match w.write_all(buf) {
        Ok(()) => 0,
        Err(err) => {
            let detail = err.raw_os_error().unwrap_or(0) as u64;
            haru::HPDF_RaiseError((*stream).error, 0x1016, detail)
        },
    }
}

