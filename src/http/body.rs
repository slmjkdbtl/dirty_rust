// wengwengweng

use crate::Result;

#[cfg(feature = "json")]
use crate::codec::json;

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

	pub fn from_text(text: &str) -> Self {
		return Self::from_bytes(text.as_bytes());
	}

	#[cfg(feature = "json")]
	pub fn from_json(json: &str) -> Result<Self> {
		return Ok(Self::from_text(&json::encode(json)?));
	}

	pub fn as_bytes(&self) -> &[u8] {
		return &self.data;
	}

	pub fn as_text(&self) -> String {
		return String::from_utf8_lossy(&self.data).to_owned().to_string();
	}

	#[cfg(feature = "json")]
	pub fn as_json<D: for<'a> serde::de::Deserialize<'a>>(&self) -> Result<D> {
		return Ok(json::decode(&self.as_text())?);
	}

	pub fn len(&self) -> usize {
		return self.data.len();
	}

}

