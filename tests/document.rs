extern crate libharu;

#[macro_use]
mod util;

use libharu::{Document, Error, PageLayout};
use std::fs::File;
use std::io;
use util::*;

#[test]
fn blank_pdf_generation() {
    assert_pdf("document_blank_pdf_generation", &mut Document::new().unwrap());
}

#[test]
fn set_pages_configuration() {
    let mut document = Document::new().unwrap();
    document.set_pages_configuration(100).unwrap();
    assert_pdf("document_set_pages_configuration", &mut document);
}

#[test]
fn page_layout() {
    let mut document = Document::new().unwrap();
    assert_eq!(document.page_layout(), PageLayout::Default);
    document.set_page_layout(PageLayout::TwoColumnLeft).unwrap();
    assert_eq!(document.page_layout(), PageLayout::TwoColumnLeft);
}

#[test]
fn set_page_layout() {
    let mut document = Document::new().unwrap();
    document.set_page_layout(PageLayout::TwoColumnLeft).unwrap();
    assert_pdf("document_set_page_layout", &mut document);
}

#[test]
fn add_page() {
    let mut document = Document::new().unwrap();
    document.add_page().unwrap();
    assert_pdf("document_add_page", &mut document);
}

#[test]
fn insert_page() {
    let mut document = Document::new().unwrap();
    let page = document.add_page().unwrap();
    document.insert_page(&page).unwrap();
    assert_pdf("document_insert_page", &mut document);
}

#[test]
fn load_ttf_font() {
    let mut document = Document::new().unwrap();
    let file = File::open(fixture_path("ttf/gohufont-11.ttf")).unwrap();
    assert!(document.load_ttf_font(file).is_ok());
}

#[test]
fn pdf_generation_io_error() {
    struct BadIo;

    impl io::Write for BadIo {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> { Err(io::Error::from_raw_os_error(7)) }
        fn flush(&mut self) -> io::Result<()> { Err(io::Error::from_raw_os_error(7)) }
    }

    expect_error!(Document::new().unwrap().save(&mut BadIo), Error::FileIo(7));
}
