use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn freeaddrinfo(emu: &mut Emu) {
    winsock::free_address_info(emu, "freeaddrinfo");
}

pub fn FreeAddrInfoW(emu: &mut Emu) {
    winsock::free_address_info(emu, "FreeAddrInfoW");
}
