use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn gethostname(emu: &mut Emu) {
    winsock::host_name(emu, "gethostname", false);
}

pub fn GetHostNameW(emu: &mut Emu) {
    winsock::host_name(emu, "GetHostNameW", true);
}
