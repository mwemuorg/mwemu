use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn getnameinfo(emu: &mut Emu) {
    winsock::name_info(emu, "getnameinfo", false);
}

pub fn GetNameInfoW(emu: &mut Emu) {
    winsock::name_info(emu, "GetNameInfoW", true);
}
