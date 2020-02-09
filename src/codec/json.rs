// wengwengweng

use serde::ser;
use serde::de;

use crate::Result;

pub fn encode<D: ser::Serialize>(data: D) -> Result<String> {
	return serde_json::to_string(&data).map_err(|_| format!("failed to encode json"));
}

pub fn decode<D: for<'a> de::Deserialize<'a>>(string: &str) -> Result<D> {
	return serde_json::from_str(&string).map_err(|_| format!("failed to decode json"));
}

