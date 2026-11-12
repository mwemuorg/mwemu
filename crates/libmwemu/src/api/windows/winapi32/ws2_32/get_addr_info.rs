use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn getaddrinfo(emu: &mut Emu) {
    winsock::address_info(emu, "getaddrinfo", false);
}

pub fn GetAddrInfoW(emu: &mut Emu) {
    winsock::address_info(emu, "GetAddrInfoW", true);
}
