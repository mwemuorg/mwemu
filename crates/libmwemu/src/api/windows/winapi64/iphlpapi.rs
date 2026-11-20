use crate::emu::Emu;
use crate::winapi::libraries;

pub fn gateway(address: u64, emu: &mut Emu) -> String {
    libraries::gateway("iphlpapi", address, emu)
}
