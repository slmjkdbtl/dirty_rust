// wengwengweng

use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
	FileWrite(PathBuf),
	FileRead(PathBuf),
	IO,
	Net,
	Image,
	Window,
	Gamepad,
	Audio,
	Parse,
	Thread,
	Lua,
	Ketos,
	Misc(String),
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return match self {
			Error::FileWrite(p) => write!(f, "failed to write {}", p.display()),
			Error::FileRead(p) => write!(f, "failed to read {}", p.display()),
			Error::IO => write!(f, "io error"),
			Error::Net => write!(f, "network error"),
			Error::Image => write!(f, "image error"),
			Error::Window => write!(f, "window error"),
			Error::Gamepad => write!(f, "gamepad error"),
			Error::Audio => write!(f, "audio error"),
			Error::Parse => write!(f, "parse error"),
			Error::Thread => write!(f, "thread error"),
			Error::Lua => write!(f, "lua error"),
			Error::Ketos => write!(f, "ketos error"),
			Error::Misc(s) => write!(f, "error: {}", s),
		};
	}
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
	fn from(_: std::io::Error) -> Self {
		return Error::IO;
	}
}

impl From<image::ImageError> for Error {
	fn from(_: image::ImageError) -> Self {
		return Error::Image;
	}
}

impl From<glutin::CreationError> for Error {
	fn from(_: glutin::CreationError) -> Self {
		return Error::Window;
	}
}

impl From<glutin::ContextError> for Error {
	fn from(_: glutin::ContextError) -> Self {
		return Error::Window;
	}
}

impl From<rodio::decoder::DecoderError> for Error {
	fn from(_: rodio::decoder::DecoderError) -> Self {
		return Error::Audio;
	}
}

impl From<serde_json::error::Error> for Error {
	fn from(_: serde_json::error::Error) -> Self {
		return Error::Parse;
	}
}

impl From<std::sync::mpsc::TryRecvError> for Error {
	fn from(_: std::sync::mpsc::TryRecvError) -> Self {
		return Error::Thread;
	}
}

impl From<ketos::Error> for Error {
	fn from(_: ketos::Error) -> Self {
		return Error::Ketos;
	}
}

impl From<ketos::ExecError> for Error {
	fn from(_: ketos::ExecError) -> Self {
		return Error::Ketos;
	}
}

impl From<gilrs::Error> for Error {
	fn from(_: gilrs::Error) -> Self {
		return Error::Thread;
	}
}

impl From<(glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)> for Error {
	fn from(_: (glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)) -> Self {
		return Error::Window;
	}
}

impl From<glob::PatternError> for Error {
	fn from(_: glob::PatternError) -> Self {
		return Error::IO;
	}
}

impl From<url::ParseError> for Error {
	fn from(_: url::ParseError) -> Self {
		return Error::Net;
	}
}

impl From<native_tls::Error> for Error {
	fn from(_: native_tls::Error) -> Self {
		return Error::Net;
	}
}

impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
	fn from(_: native_tls::HandshakeError<std::net::TcpStream>) -> Self {
		return Error::Net;
	}
}

impl From<httparse::Error> for Error {
	fn from(_: httparse::Error) -> Self {
		return Error::Net;
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		return Error::Misc(s);
	}
}

