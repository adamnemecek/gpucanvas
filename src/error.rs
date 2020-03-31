
use std::io;
use std::error::Error;
use std::ffi::NulError;
use std::fmt::{self, Display, Formatter};

use ttf_parser as ttf;

use crate::text;

/// Enum with all possible canvas errors that could occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    GeneralError(String),
    #[cfg(feature = "image-loading")]
    ImageError(::image::ImageError),
    IoError(io::Error),
    FreetypeError(text::freetype::Error),
    TtfParserError(ttf::Error),
    NoFontFound,
    FontInfoExtracionError,
    FontSizeTooLargeForAtlas,
    ShaderCompileError(String),
    ShaderLinkError(String),
    ImageIdNotFound,
    ImageUpdateOutOfBounds,
    ImageUpdateWithDifferentFormat,
    UnsuportedImageFromat,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "canvas error")
    }
}

#[cfg(feature = "image-loading")]
impl From<::image::ImageError> for ErrorKind {
    fn from(error: ::image::ImageError) -> Self {
        Self::ImageError(error)
    }
}

impl From<io::Error> for ErrorKind {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<text::freetype::Error> for ErrorKind {
    fn from(error: text::freetype::Error) -> Self {
        Self::FreetypeError(error)
    }
}

impl From<ttf::Error> for ErrorKind {
    fn from(error: ttf::Error) -> Self {
        Self::TtfParserError(error)
    }
}

impl From<NulError> for ErrorKind {
    fn from(error: NulError) -> Self {
        Self::GeneralError(error.to_string())
    }
}

impl Error for ErrorKind {}
