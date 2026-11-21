use crate::emu::Emu;
use crate::winapi::libraries;

#[cfg(test)]
pub use crate::winapi::libraries::wininet::{InternetConnectA, InternetOpenA, InternetOpenW};

pub fn gateway(address: u64, emu: &mut Emu) -> String {
    libraries::gateway("wininet", address, emu)
}
