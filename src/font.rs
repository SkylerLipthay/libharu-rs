use document::DocumentHandle;
use haru;
use std::rc::Rc;

#[derive(Clone)]
pub struct Font {
    handle: haru::HPDF_Font,
    doc: Rc<DocumentHandle>
}

/// Creates a new `Font` from a raw libharu font handle and its owner document.
#[inline]
pub fn new(font: haru::HPDF_Font, doc: Rc<DocumentHandle>) -> Font {
    Font { handle: font, doc: doc }
}

/// Extracts the libharu handle from the given `Font`.
#[inline]
pub fn get_handle(font: &Font) -> haru::HPDF_Font {
    font.handle
}
