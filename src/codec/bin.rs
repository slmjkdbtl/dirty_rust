// wengwengweng

use serde::ser;
use serde::de;

use crate::Result;

pub fn encode<D: ser::Serialize>(data: D) -> Result<Vec<u8>> {
	return bincode::serialize(&data).map_err(|_| format!("failed to encode bin"));
}

pub fn decode<D: for<'a> de::Deserialize<'a>>(data: &[u8]) -> Result<D> {
	return bincode::deserialize(&data).map_err(|_| format!("failed to decode bin"));
}

