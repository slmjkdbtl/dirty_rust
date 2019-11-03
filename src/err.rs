// wengwengweng

use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
	Fs(String),
	Net(String),
	Image(String),
	Window(String),
	Web(String),
	Audio(String),
	Thread(String),
	Gfx(String),
	Obj(String),
	Gltf(String),
	Input(String),
	OpenGL(String),
	Json(String),
	Bin(String),
	Misc(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let print_err = |f: &mut fmt::Formatter, ty: &str, msg: &str| -> std::fmt::Result {
			if msg.is_empty() {
				return write!(f, "{} error", ty);
			} else {
				return write!(f, "{} error: {}", ty, msg);
			}
		};

		return match self {
			Error::Fs(s) => print_err(f, "fs", s),
			Error::Net(s) => print_err(f, "network", s),
			Error::Image(s) => print_err(f, "image", s),
			Error::Window(s) => print_err(f, "window", s),
			Error::Web(s) => print_err(f, "wasm", s),
			Error::Audio(s) => print_err(f, "audio", s),
			Error::Thread(s) => print_err(f, "thread", s),
			Error::Gfx(s) => print_err(f, "gfx", s),
			Error::Obj(s) => print_err(f, "obj", s),
			Error::Gltf(s) => print_err(f, "gltf", s),
			Error::Input(s) => print_err(f, "input", s),
			Error::OpenGL(s) => print_err(f, "opengl", s),
			Error::Json(s) => print_err(f, "json", s),
			Error::Bin(s) => print_err(f, "bin", s),
			Error::Misc(s) => print_err(f, "misc", s),
		};

	}

}

// TODO
impl From<std::io::Error> for Error {
	fn from(_: std::io::Error) -> Self {
		return Error::Misc(format!(""));
	}
}

impl From<std::sync::mpsc::TryRecvError> for Error {
	fn from(_: std::sync::mpsc::TryRecvError) -> Self {
		return Error::Thread(format!("failed to receive from another thread"));
	}
}

impl From<std::string::FromUtf8Error> for Error {
	fn from(_: std::string::FromUtf8Error) -> Self {
		return Error::Misc("failed to convert utf8 to string".into());
	}
}

impl From<String> for Error {
	fn from(s: String) -> Self {
		return Error::Misc(s);
	}
}

impl From<()> for Error {
	fn from(_: ()) -> Self {
		return Error::Misc(String::new());
	}
}

#[cfg(feature = "img")]
impl From<image::ImageError> for Error {

	fn from(err: image::ImageError) -> Self {

		use image::ImageError::*;

		let msg = match err {
			FormatError(..) => "format error".into(),
			DimensionError => "incorrect image dimension".into(),
			UnsupportedError(fmt) => format!("{} not supported", fmt),
			UnsupportedColor(c) => format!("{:?} not supported", c),
			NotEnoughData => "not enough data provided to decode the image".into(),
			IoError(..) => "io".into(),
			ImageEnd => "the end of image has been reached".into(),
			InsufficientMemory => "not enough memory".into(),
		};

		return Error::Image(msg);

	}

}

#[cfg(all(feature = "app", not(web)))]
impl From<glutin::CreationError> for Error {

	fn from(err: glutin::CreationError) -> Self {

		use glutin::CreationError::*;

		let msg = match err {
			OsError(..) => "".into(),
			NotSupported(..) => "".into(),
			NoBackendAvailable(..) => "no backend available".into(),
			RobustnessNotSupported => "robustness not supported".into(),
			OpenGlVersionNotSupported => "opengl version not supported".into(),
			NoAvailablePixelFormat => "pixel format not available".into(),
			PlatformSpecific(s) => format!("{}", s),
			Window(..) => "window creation error".into(),
			CreationErrors(..) => "window creation error".into(),
		};

		return Error::Window(msg);

	}

}

#[cfg(all(feature = "app", not(web)))]
impl From<glutin::ContextError> for Error {
	fn from(_: glutin::ContextError) -> Self {
		return Error::Window("failed to create window context".into());
	}
}

#[cfg(all(feature = "app", not(web)))]
impl From<(glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)> for Error {
	fn from(_: (glutin::ContextWrapper<glutin::NotCurrent, glutin::Window>, glutin::ContextError)) -> Self {
		return Error::Window("failed to create window context".into());
	}
}

#[cfg(all(feature = "app", web))]
impl From<stdweb::web::error::InvalidCharacterError> for Error {
	fn from(_: stdweb::web::error::InvalidCharacterError) -> Self {
		return Error::Web(format!("invalid character"));
	}
}

#[cfg(all(feature = "app", web))]
impl From<stdweb::serde::ConversionError> for Error {
	fn from(_: stdweb::serde::ConversionError) -> Self {
		return Error::Web(format!("conversion"));
	}
}

#[cfg(feature = "audio")]
impl From<rodio::decoder::DecoderError> for Error {
	fn from(_: rodio::decoder::DecoderError) -> Self {
		return Error::Audio("failed to decode".into());
	}
}

#[cfg(all(feature = "app", not(mobile), not(web)))]
impl From<gilrs::Error> for Error {
	fn from(_: gilrs::Error) -> Self {
		return Error::Input("gamepad error".into());
	}
}

#[cfg(feature = "app")]
impl From<tobj::LoadError> for Error {

	fn from(err: tobj::LoadError) -> Self {

		use tobj::LoadError::*;

		let msg = match err {
			OpenFileFailed => "".into(),
			ReadError => "".into(),
			UnrecognizedCharacter => "unrecognized character".into(),
			PositionParseError => "failed to parse positions".into(),
			NormalParseError => "failed to parse normals".into(),
			TexcoordParseError => "failed to parse tex coords".into(),
			FaceParseError => "failed to parse faces".into(),
			MaterialParseError => "failed to parse material".into(),
			InvalidObjectName => "invalid object name".into(),
			GenericFailure => "unknown error".into(),
		};

		return Error::Obj(msg);

	}

}

#[cfg(all(feature = "app"))]
impl From<gltf::Error> for Error {
	fn from(_: gltf::Error) -> Self {
		return Error::Gltf(format!(""));
	}
}

#[cfg(feature = "http")]
impl From<url::ParseError> for Error {
	fn from(_: url::ParseError) -> Self {
		return Error::Net("failed to parse url".into());
	}
}

#[cfg(feature = "http")]
impl From<httparse::Error> for Error {

	fn from(err: httparse::Error) -> Self {

		use httparse::Error::*;

		let msg = match err {
			HeaderName => "invalid header name".into(),
			HeaderValue => "invalid header value".into(),
			NewLine => "invalid byte in new line".into(),
			Status => "invalid response status".into(),
			Token => "invalid byte where token is required".into(),
			TooManyHeaders => "too many headers".into(),
			Version => "invalid http version".into(),
		};

		return Error::Net(msg);

	}

}

#[cfg(all(feature = "http", not(mobile), not(web)))]
impl From<native_tls::Error> for Error {
	fn from(_: native_tls::Error) -> Self {
		return Error::Net("tls error".into());
	}
}

#[cfg(all(feature = "http", not(mobile), not(web)))]
impl From<native_tls::HandshakeError<std::net::TcpStream>> for Error {
	fn from(_: native_tls::HandshakeError<std::net::TcpStream>) -> Self {
		return Error::Net("tls handshake error".into());
	}
}

#[cfg(feature = "json")]
impl From<serde_json::Error> for Error {
	fn from(e: serde_json::Error) -> Self {
		return Error::Json(format!("line {}", e.line()));
	}
}

#[cfg(feature = "bin")]
impl From<Box<bincode::ErrorKind>> for Error {
	fn from(e: Box<bincode::ErrorKind>) -> Self {
		return Error::Bin(format!("{}", e));
	}
}

impl From<Box<dyn std::error::Error>> for Error {
	fn from(_: Box<dyn std::error::Error>) -> Self {
		return Error::Misc(format!(""));
	}
}

impl From<&str> for Error {
	fn from(s: &str) -> Self {
		return Error::Misc(format!("{}", s));
	}
}

