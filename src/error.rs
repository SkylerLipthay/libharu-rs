use haru::HPDF_STATUS;
use std::ffi::NulError;

/// Represents all possible errors from libharu.
///
/// Many of these errors will never occur in practice because of the limitations or safety of this
/// binding, or because some errors are deprecated or otherwise unused by libharu.
#[derive(Debug)]
pub enum Error {
    /// Internal error. Data consistency was lost.
    ArrayCount,
    /// Internal error. Data consistency was lost.
    ArrayItemNotFound,
    /// Internal error. Data consistency was lost.
    ArrayItemUnexpectedType,
    /// Data length > 65535.
    BinaryLength,
    /// Dictionary elements > 4095
    DictionaryCount,
    /// Internal error. Data consistency was lost.
    DictionaryItemNotFound,
    /// Internal error. Data consistency was lost.
    DictionaryItemUnexpectedType,
    /// Internal error. Data consistency was lost.
    DictionaryStreamLengthNotFound,
    /// `Document::set_encryption_mode` or `Document::set_permission` called before password set.
    DocumentEncryptionDictionaryNotFound,
    /// Internal error. Data consistency was lost.
    DocumentInvalidObject,
    /// Tried to re-register a registered font.
    DuplicateRegistration,
    /// Cannot register a character to the Japanese word wrap characters list.
    JwwCodeNumberLimitExceeded(u64),
    /// * Tried to set the owner password to NULL.
    /// * Owner and user password are the same.
    EncryptInvalidPassword,
    /// Internal error. Data consistency was lost.
    UnknownClass,
    /// Stack depth > 28.
    GstateLimitExceeded,
    /// Memory allocation failed.
    AllocationFailed,
    /// File processing failed. Check the `Error`'s `detail` for more information.
    FileIo(u64),
    /// Cannot open a file. Check the `Error`'s `detail` for more information.
    FileOpen(u64),
    /// Tried to load a font that has been registered.
    FontExists,
    /// * Font-file format is invalid.
    /// * Internal error. Data consistency was lost.
    FontInvalidWidthsTable,
    /// Cannot recognize header of AFM file.
    InvalidAfmHeader,
    /// Specified annotation handle is invalid.
    InvalidAnnotation,
    /// Bit-per-component of a image which was set as mask-image is invalid.
    InvalidBitPerComponent,
    /// Cannot recognize char-matrics-data of AFM file.
    InvalidCharMatricsData,
    /// * Invalid color_space parameter of `Document::load_raw_image_from_file` or
    ///   `Document::load_raw_image_from_mem`.
    /// * Color-space of a image which was set as mask-image is invalid.
    /// * Invoked function invalid in present color-space.
    InvalidColorSpace,
    /// Invalid value set when invoking `Document::set_compression_mode`.
    InvalidCompressionMode,
    /// An invalid date-time value was set.
    InvalidDateTime,
    /// An invalid destination handle was set.
    InvalidDestination,
    /// An invalid document handle was set.
    InvalidDocument,
    /// Function invalid in the present state was invoked.
    InvalidDocumentState,
    /// An invalid encoder handle was set.
    InvalidEncoder,
    /// Combination between font and encoder is wrong.
    InvalidEncoderType,
    /// An Invalid encoding name is specified.
    InvalidEncodingName,
    /// Encryption key length is invalid.
    InvalidEncryptKeyLen,
    /// * An invalid font handle was set.
    /// * Unsupported font format.
    InvalidFontdefData,
    /// Internal error. Data consistency was lost.
    InvalidFontdefType,
    /// Font with the specified name is not found.
    InvalidFontName,
    /// Unsupported image format.
    InvalidImage,
    /// Unsupported image format.
    InvalidJpegData,
    /// Cannot read a postscript-name from an AFM file.
    InvalidNData,
    /// * An invalid object is set.
    /// * Internal error. Data consistency was lost.
    InvalidObject,
    /// Internal error. Data consistency was lost.
    InvalidObjectId,
    /// Invoked `Image::set_color_mask` against the image-object which was set a mask-image.
    InvalidOperation,
    /// An invalid outline-handle was specified.
    InvalidOutline,
    /// An invalid page-handle was specified.
    InvalidPage,
    /// An invalid pages-handle was specified (internal error).
    InvalidPages,
    /// An invalid value is set.
    InvalidParameter,
    /// Invalid PNG image format.
    InvalidPngImage,
    /// Internal error. Data consistency was lost.
    InvalidStream,
    /// Internal error. "_FILE_NAME" entry for delayed loading is missing.
    MissingFileNameEntry,
    /// Invalid TTC file format.
    InvalidTtcFile,
    /// Index parameter > number of included fonts.
    InvalidTtcIndex,
    /// Cannot read a width-data from an AFM file.
    InvalidWxData,
    /// Internal error. Data consistency was lost.
    ItemNotFound,
    /// Error returned from libpng while loading image.
    Libpng(u64),
    /// Cannot get palette data from PNG image.
    CannotGetPalette,
    /// Internal error. Data consistency was lost.
    NameInvalidValue,
    /// Internal error. Data consistency was lost.
    NameOutOfRange,
    /// An invalid number of parameters was passed to a page operation.
    PageInvalidParameterCount,
    /// Internal error. Data consistency was lost.
    PagesMissingKidsEntry,
    /// Internal error. Data consistency was lost.
    PageCannotFindObject,
    /// Internal error. Data consistency was lost.
    PageCannotGetRootPages,
    /// There are no graphics-states to be restored.
    PageCannotRestoreGstate,
    /// Internal error. Data consistency was lost.
    PageCannotSetParent,
    /// The current font is not set.
    PageFontNotFound,
    /// An invalid font-handle was specified.
    PageInvalidFont,
    /// An invalid font-size was set.
    PageInvalidFontSize,
    /// An operation was unable to be executed in the current graphics mode. This will occur, for
    /// example, if `Page::set_gray_stroke` is called in between `Page::line_to` calls.
    ///
    /// Consult [libharu's manual](https://github.com/libharu/libharu/wiki/Graphics#graphics-mode)
    /// for full documentation of the graphics mode.
    PageInvalidGmode,
    /// Internal error. Data consistency was lost.
    PageInvalidIndex,
    /// Specified value is not multiple of 90.
    PageInvalidRotateValue,
    /// An invalid page-size was set.
    PageInvalidSize,
    /// An invalid image-handle was set.
    PageInvalidXobject,
    /// The specified value is out of range.
    PageOutOfRange,
    /// The specified value is out of range.
    RealOutOfRange,
    /// Unexpected EOF marker was detected.
    StreamEof,
    /// Internal error. Data consistency was lost.
    StreamReadlnContinue,
    /// The length of the text is too long.
    StringOutOfRange,
    /// Function not executed because of other errors.
    FunctionSkipped,
    /// Font cannot be embedded (license restriction).
    TtfCannotEmbeddingFont,
    /// Unsupported TTF format (cannot find unicode cmap).
    TtfInvalidCmap,
    /// Unsupported TTF format.
    TtfInvalidFomat,
    /// Unsupported TTF format (cannot find a necessary table).
    TtfMissingTable(u64),
    /// Internal error. Data consistency was lost.
    UnsupportedFontType,
    /// * Library not configured to use PNGLIB.
    /// * Internal error. Data consistency was lost.
    FunctionUnsupported,
    /// Unsupported JPEG format.
    UnsupportedJpegFormat,
    /// Failed to parse PFB file.
    UnsupportedType1Font,
    /// Internal error. Data consistency was lost.
    XrefCountErr,
    /// Error while executing zlib function.
    Zlib(u64),
    /// An invalid page index was passed.
    InvalidPageIndex,
    /// An invalid URI was set.
    InvalidUri,
    /// An invalid page-layout was set.
    PageLayoutOutOfRange,
    /// An invalid page-mode was set.
    PageModeOutOfRange,
    /// An invalid page-num-style was set.
    PageNumberStyleOutOfRange(u64),
    /// An invalid icon was set.
    AnnotationInvalidIcon,
    /// An invalid border-style was set.
    AnnotationInvalidBorderStyle,
    /// An invalid page-direction was set.
    PageInvalidDirection(u64),
    /// An invalid font-handle was specified.
    InvalidFont,
    /// Page has insufficient space for the operation to succeed.
    PageInsufficientSpace,
    /// Page slideshow display time was invalid.
    PageInvalidDisplayTime,
    /// Page slideshow transition time was invalid.
    PageInvalidTransitionTime,
    /// Page slideshow type was invalid.
    PageInvalidSlideshowType,
    /// An argument passed to a graphics-state operation was out of valid range.
    ExtGstateOutOfRange,
    /// The state of the graphics-state during an operation was invalid.
    ExtGstateInvalid,
    /// The state of the graphics-state during an operation was read-only.
    ExtGstateReadOnly,
    /// Universal 3D data was not well-formatted.
    U3dDataInvalid,
    /// Failed to get item names.
    CannotGetNames,
    /// Unsupported ICC format.
    IccComponentCountInvalid,
    /// The provided string contained a 0 (NUL) byte, so it could not be converted into a C string
    /// consumable by libharu.
    StringWithInternalNul,
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Error {
        Error::StringWithInternalNul
    }
}

/// Accepts two native libharu statuses (a main one and an optional detail) and returns a possible
/// `Error`. If `status` is a successful code, `detail` is ignored and `None` is returned.
///
/// `detail` is only utilized for a few errors.
///
/// # Panics
///
/// Panics if either of the provided statuses are unrecognized codes. If this panic occurs, it is
/// most likely an internal libharu bug.
pub fn from(status: HPDF_STATUS, detail: HPDF_STATUS) -> Result<(), Error> {
    use Error::*;

    Err(match status {
        0 => return Ok(()),
        0x1001 => ArrayCount,
        0x1002 => ArrayItemNotFound,
        0x1003 => ArrayItemUnexpectedType,
        0x1004 => BinaryLength,
        0x1007 => DictionaryCount,
        0x1008 => DictionaryItemNotFound,
        0x1009 => DictionaryItemUnexpectedType,
        0x100a => DictionaryStreamLengthNotFound,
        0x100b => DocumentEncryptionDictionaryNotFound,
        0x100c => DocumentInvalidObject,
        0x100e => DuplicateRegistration,
        0x100f => JwwCodeNumberLimitExceeded(detail),
        0x1011 => EncryptInvalidPassword,
        0x1013 => UnknownClass,
        0x1014 => GstateLimitExceeded,
        0x1015 => AllocationFailed,
        0x1016 => FileIo(detail),
        0x1017 => FileOpen(detail),
        0x1019 => FontExists,
        0x101a => FontInvalidWidthsTable,
        0x101b => InvalidAfmHeader,
        0x101c => InvalidAnnotation,
        0x101e => InvalidBitPerComponent,
        0x101f => InvalidCharMatricsData,
        0x1020 => InvalidColorSpace,
        0x1021 => InvalidCompressionMode,
        0x1022 => InvalidDateTime,
        0x1023 => InvalidDestination,
        0x1025 => InvalidDocument,
        0x1026 => InvalidDocumentState,
        0x1027 => InvalidEncoder,
        0x1028 => InvalidEncoderType,
        0x102b => InvalidEncodingName,
        0x102c => InvalidEncryptKeyLen,
        0x102d => InvalidFontdefData,
        0x102e => InvalidFontdefType,
        0x102f => InvalidFontName,
        0x1030 => InvalidImage,
        0x1031 => InvalidJpegData,
        0x1032 => InvalidNData,
        0x1033 => InvalidObject,
        0x1034 => InvalidObjectId,
        0x1035 => InvalidOperation,
        0x1036 => InvalidOutline,
        0x1037 => InvalidPage,
        0x1038 => InvalidPages,
        0x1039 => InvalidParameter,
        0x103b => InvalidPngImage,
        0x103c => InvalidStream,
        0x103d => MissingFileNameEntry,
        0x103f => InvalidTtcFile,
        0x1040 => InvalidTtcIndex,
        0x1041 => InvalidWxData,
        0x1042 => ItemNotFound,
        0x1043 => match detail {
            0x1005 => CannotGetPalette,
            code => Libpng(code),
        },
        0x1044 => NameInvalidValue,
        0x1045 => NameOutOfRange,
        0x1048 => PageInvalidParameterCount,
        0x1049 => PagesMissingKidsEntry,
        0x104a => PageCannotFindObject,
        0x104b => PageCannotGetRootPages,
        0x104c => PageCannotRestoreGstate,
        0x104d => PageCannotSetParent,
        0x104e => PageFontNotFound,
        0x104f => PageInvalidFont,
        0x1050 => PageInvalidFontSize,
        0x1051 => PageInvalidGmode,
        0x1052 => PageInvalidIndex,
        0x1053 => PageInvalidRotateValue,
        0x1054 => PageInvalidSize,
        0x1055 => PageInvalidXobject,
        0x1056 => PageOutOfRange,
        0x1057 => RealOutOfRange,
        0x1058 => StreamEof,
        0x1059 => StreamReadlnContinue,
        0x105b => StringOutOfRange,
        0x105c => FunctionSkipped,
        0x105d => TtfCannotEmbeddingFont,
        0x105e => TtfInvalidCmap,
        0x105f => TtfInvalidFomat,
        0x1060 => TtfMissingTable(detail),
        0x1061 => UnsupportedFontType,
        0x1062 => FunctionUnsupported,
        0x1063 => UnsupportedJpegFormat,
        0x1064 => UnsupportedType1Font,
        0x1065 => XrefCountErr,
        0x1066 => Zlib(detail),
        0x1067 => InvalidPageIndex,
        0x1068 => InvalidUri,
        0x1069 => PageLayoutOutOfRange,
        0x1070 => PageModeOutOfRange,
        0x1071 => PageNumberStyleOutOfRange(detail),
        0x1072 => AnnotationInvalidIcon,
        0x1073 => AnnotationInvalidBorderStyle,
        0x1074 => PageInvalidDirection(detail),
        0x1075 => InvalidFont,
        0x1076 => PageInsufficientSpace,
        0x1077 => PageInvalidDisplayTime,
        0x1078 => PageInvalidTransitionTime,
        0x1079 => PageInvalidSlideshowType,
        0x1080 => ExtGstateOutOfRange,
        0x1081 => ExtGstateInvalid,
        0x1082 => ExtGstateReadOnly,
        0x1083 => U3dDataInvalid,
        0x1084 => CannotGetNames,
        0x1085 => IccComponentCountInvalid,
        v => panic!("Invalid error status from libharu {:x}", v),
    })
}

#[test]
fn error_translation() {
    macro_rules! expect_error {
        ($val:expr, $err:pat) => {
            match $val {
                Err($err) => (),
                _ => panic!("Invalid error"),
            }
        }
    }

    assert!(from(0, 0).is_ok());
    expect_error!(from(0x1075, 0), Error::InvalidFont);
    expect_error!(from(0x1060, 9), Error::TtfMissingTable(9));
}
