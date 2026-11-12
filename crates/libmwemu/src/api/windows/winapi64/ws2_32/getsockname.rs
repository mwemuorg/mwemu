use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn getsockname(emu: &mut Emu) {
    winsock::socket_name(emu);
}
