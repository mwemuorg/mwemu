use crate::emu::Emu;
use crate::winapi::libraries;

#[cfg(test)]
pub use crate::winapi::libraries::winhttp::{
    WinHttpCloseHandle, WinHttpConnect, WinHttpOpen, WinHttpOpenRequest, WinHttpReadData,
    WinHttpReceiveResponse, WinHttpSendRequest,
};

pub fn gateway(address: u64, emu: &mut Emu) -> String {
    libraries::gateway("winhttp", address, emu)
}
