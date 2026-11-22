use crate::emu::Emu;
use crate::winapi::libraries;

pub use crate::winapi::libraries::oleaut32::exports::*;

pub fn gateway(address: u32, emu: &mut Emu) -> String {
    libraries::dispatch_address(address as u64, emu, dispatch)
}
