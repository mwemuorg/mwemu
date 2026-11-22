use crate::emu::Emu;
use crate::winapi::libraries;

pub fn gateway(address: u32, emu: &mut Emu) -> String {
    libraries::gateway("urlmon", address as u64, emu)
}
