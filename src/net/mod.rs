// wengwengweng

//! Basic Networking

use std::net::UdpSocket;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;

use crate::*;

const RECV_BUF_SIZE: usize = 1024;

pub struct Client {
	socket: UdpSocket,
	recv_buf: [u8; RECV_BUF_SIZE],
}

impl Client {

	pub fn new(addr: impl ToSocketAddrs) -> Result<Self> {
		return Ok(Self {
			socket: UdpSocket::bind(addr)
				.map_err(|_| format!("failed to bind to udp socket"))?,
			recv_buf: [0; RECV_BUF_SIZE],
		});
	}

	pub fn send(&self, dest: impl ToSocketAddrs, payload: &[u8]) -> Result<()> {
		self.socket
			.send_to(payload, dest)
			.map_err(|_| format!("failed to send packet"))?;
		return Ok(());
	}

	pub fn recv(&mut self) -> Result<(&[u8], SocketAddr)> {
		return self.socket
			.recv_from(&mut self.recv_buf)
			.map(move |(size, addr)| (&self.recv_buf[..size], addr))
			.map_err(|_| format!("failed to recv packet"));
	}

}

