// wengwengweng

use serde::ser;
use serde::de;

use crate::Result;

pub fn encode<D: ser::Serialize>(data: D) -> Result<Vec<u8>> {
	return Ok(bincode::serialize(&data)?);
}

pub fn decode<D: for<'a> de::Deserialize<'a>>(data: &[u8]) -> Result<D> {
	return Ok(bincode::deserialize(&data)?);
}

