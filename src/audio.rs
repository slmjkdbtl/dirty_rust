// wengwengweng

//! Audio

#[cfg(not(web))]
mod native;
#[cfg(web)]
mod web;

