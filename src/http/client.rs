// wengwengweng

use crate::Result;
use super::*;

pub fn get(url: &str) -> Result<Response> {
	return Request::get(url)?.send(None);
}

pub fn post(url: &str, data: impl AsRef<[u8]>) -> Result<Response> {
	return Request::post(url)?.send(Some(data.as_ref()));
}

