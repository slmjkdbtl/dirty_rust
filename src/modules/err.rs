// wengwengweng

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
	IO,
	Net,
	Image,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::IO => write!(f, "io error"),
			Error::Net => write!(f, "network error"),
			Error::Image => write!(f, "image error"),
		};
	}
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
	fn from(_: std::io::Error) -> Self {
		return Error::IO;
	}
}

impl From<reqwest::Error> for Error {
	fn from(_: reqwest::Error) -> Self {
		return Error::Net;
	}
}

impl From<image::ImageError> for Error {
	fn from(_: image::ImageError) -> Self {
		return Error::Image;
	}
}

