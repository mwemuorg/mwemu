use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn WSAStartup(emu: &mut Emu) {
    winsock::startup(emu);
}
