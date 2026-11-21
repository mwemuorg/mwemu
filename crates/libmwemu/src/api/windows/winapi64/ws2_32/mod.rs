use crate::emu::Emu;
use crate::winapi::libraries;

#[cfg(test)]
pub use crate::winapi::libraries::ws2_32::{WSAStartup, getaddrinfo, htons};

pub fn gateway(address: u64, emu: &mut Emu) -> String {
    libraries::gateway("ws2_32", address, emu)
}
