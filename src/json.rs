// wengwengweng

use serde::ser;
use serde::de;
pub use serde::Serialize;
pub use serde::Deserialize;

use crate::Result;

pub fn encode<D: ser::Serialize>(data: D) -> Result<String> {
	return Ok(serde_json::to_string(&data)?);
}

pub fn decode<D: for<'a> de::Deserialize<'a>>(string: &str) -> Result<D> {
	return Ok(serde_json::from_str(&string)?);
}

