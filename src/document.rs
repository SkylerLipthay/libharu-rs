use error::{self, Error};
use font::{self, Font};
use haru;
use page::{self, Page};
use std::io::{Read, Seek, Write};
use std::ptr;
use std::rc::Rc;
use stream;
use types::{self, PageLayout};

/// A PDF document.
pub struct Document {
    inner: Rc<DocumentHandle>,
}

impl Document {
    /// Creates an instance of a document object and initializes it.
    pub fn new() -> Result<Document, Error> {
        let handle_ptr = unsafe { haru::HPDF_New(None, ptr::null_mut()) };
        if handle_ptr == ptr::null_mut() {
            return Err(Error::AllocationFailed);
        }

        let handle = DocumentHandle(handle_ptr);
        try!(handle.check_error(unsafe { haru::HPDF_UseUTFEncodings(handle.0) }));
        Ok(Document { inner: Rc::new(handle) })
    }

    /// Writes the PDF to the given `Write` stream, returning an error if the PDF could not be
    /// generated or if any I/O operation fails.
    pub fn save<W: Write>(&mut self, w: &mut W) -> Result<(), Error> {
        let stream = unsafe { stream::convert_write_stream(&*self.inner, w) };
        let status = unsafe { haru::HPDF_SaveToExternalStream(self.inner.0, stream) };
        // Stream must be manually consumed.
        unsafe { haru::HPDF_Stream_Free(stream); }
        self.inner.check_error(status)
    }

    /// Sets the number of maximum number of "Pages" objects of the root "Pages" object.
    ///
    /// By default, a document object has one "Pages" object as the root of all pages. All "Page"
    /// objects are created as children of this "Pages" object. Since a "Pages" object can own only
    /// 8191 children objects, the maximum number of pages are 8191 page.
    ///
    /// Using this method allows the root "Pages" object, instead of having up to 8191 "Page"
    /// children, to have `page_per_pages` "Pages" children, who in turn have up to 8191 "Page"
    /// children each. As a result, the total maximum number of pages becomes 8191 *
    /// `page_per_pages`.
    ///
    /// This operation will result in an error if any pages have already been added to this
    /// document.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_pages_configuration(&mut self, page_per_pages: u32) -> Result<&mut Self, Error> {
        let result = unsafe { haru::HPDF_SetPagesConfiguration(self.inner.0, page_per_pages) };
        try!(self.inner.check_error(result));
        Ok(self)
    }

    /// Returns the page layout option for this document.
    pub fn page_layout(&self) -> PageLayout {
        types::page_layout_from_int(unsafe { haru::HPDF_GetPageLayout(self.inner.0) })
    }

    /// Sets the page layout option for this document.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_page_layout(&mut self, layout: PageLayout) -> Result<&mut Self, Error> {
        let layout = types::page_layout_as_int(layout);
        try!(self.inner.check_error(unsafe { haru::HPDF_SetPageLayout(self.inner.0, layout) }));
        Ok(self)
    }

    /// Creates a new page, adds it after the last page of a document, the returns it.
    pub fn add_page(&mut self) -> Result<Page, Error> {
        let handle = try!(self.inner.check_non_null_mut(unsafe {
            haru::HPDF_AddPage(self.inner.0)
        }));
        Ok(page::new(handle, self.inner.clone()))
    }

    /// Creates a new page, inserts it just before the specified page, then returns it.
    pub fn insert_page(&mut self, before: &Page) -> Result<Page, Error> {
        let handle_ptr = unsafe { haru::HPDF_InsertPage(self.inner.0, page::get_handle(before)) };
        let handle = try!(self.inner.check_non_null_mut(handle_ptr));
        Ok(page::new(handle, self.inner.clone()))
    }

    /// Reads and loads a TTF font from the given stream.
    pub fn load_ttf_font<R: Read + Seek>(&mut self, r: R) -> Result<Font, Error> {
        let name = try!(self.inner.check_non_null(unsafe {
            let stream = stream::convert_read_stream(&*self.inner, r);
            // `haru::HPDF_LoadTTFontFromStream` consumes the stream.
            haru::HPDF_LoadTTFontFromStream(self.inner.0, stream, 1, ptr::null())
        }));

        let handle = try!(self.inner.check_non_null_mut(unsafe {
            haru::HPDF_GetFont(self.inner.0, name, b"UTF-8".as_ptr() as *const i8)
        }));

        Ok(font::new(handle, self.inner.clone()))
    }
}

/// A wrapper around a raw libharu handle for a document.
///
/// The internal handle is freed when the `DocumentHandle` instance is dropped.
///
/// # Warning
///
/// If an error occurs in libharu without passing through either `check_error` or `check_non_null`,
/// `haru::HPDF_ResetError` must be called manually elsewhere. More specifically, if
/// `haru::HPDF_ResetError` is not called after an error occurs, most operations on the
/// corresponding document and its child objects will fail indiscriminately. It is best to run all
/// possible errors (status values, null pointer return values, etc.) through `DocumentHandle`'s
/// error handling methods.
pub struct DocumentHandle(pub haru::HPDF_Doc);

impl DocumentHandle {
    /// Returns an `Error` if the given status is not a successful code.
    ///
    /// If there is an error, this method automatically retrieves and sets the detail error code,
    /// then resets the error so libharu can function normally.
    pub fn check_error(&self, status: haru::HPDF_STATUS) -> Result<(), Error> {
        error::from(status, match status {
            0 => 0,
            _ => unsafe {
                let detail = haru::HPDF_GetErrorDetail(self.0);
                haru::HPDF_ResetError(self.0);
                detail
            },
        })
    }

    /// If the pointer is null, this method retrieves the latest error and resets the error so
    /// libharu can function normally. This method returns the pointer itself if it is non-null.
    pub fn check_non_null<T>(&self, p: *const T) -> Result<*const T, Error> {
        if p == ptr::null_mut() {
            match self.check_error(unsafe { haru::HPDF_GetError(self.0) }) {
                // There is no reported error from libharu, but a null pointer was still returned.
                // Maybe it's an allocation failure?
                Ok(()) => Err(Error::AllocationFailed),
                Err(err) => Err(err),
            }
        } else {
            Ok(p)
        }
    }

    /// A variation of `check_non_null` for mutable pointers.
    pub fn check_non_null_mut<T>(&self, p: *mut T) -> Result<*mut T, Error> {
        self.check_non_null(p as *const _).map(|p| p as *mut _)
    }
}

impl Drop for DocumentHandle {
    fn drop(&mut self) {
        unsafe { haru::HPDF_Free(self.0); }
    }
}
