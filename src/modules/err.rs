// wengwengweng

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
	IO,
	Net,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::IO => write!(f, "io error"),
			Error::Net => write!(f, "network error"),
		};
	}
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Error {
		return Error::IO;
	}
}

impl From<reqwest::Error> for Error {
	fn from(err: reqwest::Error) -> Error {
		return Error::Net;
	}
}

