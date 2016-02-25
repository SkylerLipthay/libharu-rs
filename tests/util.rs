#![allow(dead_code)]

extern crate libharu;

use libharu::Document;
use std::env;
use std::fs::File;
use std::io::Read;

/// Panics if the `Result` `$val` does not match the pattern `Err($err)`.
macro_rules! expect_error {
    ($val:expr, $err:pat) => {
        match $val {
            Err($err) => (),
            _ => panic!("Invalid error {:?}", $val),
        }
    }
}

/// A helper that creates a PDF file in the tests directory for manual inspection. If the resultant
/// PDF file passes the test author's verification it is to be tested against permanently.
///
/// The file is written to `/tests/fixtures/[name].pdf`.
///
/// Usage of this function should never be committed to source control.
pub fn create_pdf(name: &str, document: &mut Document) {
    document.save(&mut File::create(&pdf_path(name)).unwrap()).unwrap();
}

/// Panics if the bytes of the PDF file at `/tests/fixtures/[name].pdf` do not match the provided
/// `data`.
pub fn assert_pdf(name: &str, document: &mut Document) {
    let mut desired = vec![];
    let mut actual = vec![];
    File::open(&pdf_path(name)).unwrap().read_to_end(&mut desired).unwrap();
    assert!(document.save(&mut actual).is_ok());
    if desired != actual {
        panic!("{}.pdf failed assertion", name);
    }
}

/// Returns the path to a file in the test fixtures directory.
pub fn fixture_path(file_name: &str) -> String {
    format!("{}/tests/fixtures/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), file_name)
}

fn pdf_path(file_name: &str) -> String {
    fixture_path(&format!("pdf/{}.pdf", file_name))
}
