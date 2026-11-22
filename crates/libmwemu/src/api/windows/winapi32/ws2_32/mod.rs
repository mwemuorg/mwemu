use crate::emu::Emu;
use crate::winapi::libraries;

#[cfg(test)]
pub use crate::winapi::libraries::ws2_32::{WSAStartup, htons};

pub fn gateway(address: u32, emu: &mut Emu) -> String {
    libraries::gateway("ws2_32", address as u64, emu)
}
