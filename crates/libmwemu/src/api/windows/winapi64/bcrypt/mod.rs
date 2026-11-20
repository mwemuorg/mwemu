use crate::emu::Emu;
use crate::winapi::libraries;

pub use crate::winapi::libraries::bcrypt::exports::*;

pub fn gateway(address: u64, emu: &mut Emu) -> String {
    libraries::dispatch_address(address, emu, dispatch)
}
