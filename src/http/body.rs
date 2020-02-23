// wengwengweng

#[cfg(feature = "json")]
use crate::codec::json;
use crate::Result;

#[derive(Clone, Debug)]
pub struct Body {
	data: Vec<u8>,
}

impl Body {

	pub fn empty() -> Self {
		return Self {
			data: vec![],
		};
	}

	pub fn from_bytes(data: &[u8]) -> Self {
		return Self {
			data: data.to_owned(),
		};
	}

	pub fn from_string(data: &str) -> Self {
		return Self {
			data: data.to_owned().into_bytes(),
		};
	}

	#[cfg(feature = "json")]
	pub fn from_json<D: serde::ser::Serialize>(data: D) -> Result<Self> {
		return Ok(Self {
			data: json::encode(data)?.into_bytes(),
		});
	}

	pub fn into_bytes(self) -> Vec<u8> {
		return self.data;
	}

	pub fn into_string(self) -> String {
		return String::from_utf8_lossy(&self.data)
			.to_string();
	}

	#[cfg(feature = "json")]
	pub fn into_json<D: for<'a> serde::de::Deserialize<'a>>(self) -> Result<D> {
		return Ok(json::decode(&self.into_string())?);
	}

	pub fn len(&self) -> usize {
		return self.data.len();
	}

}

