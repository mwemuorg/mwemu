use crate::emu::Emu;
use crate::winapi::common::winsock::sockets;

pub fn WSASocketA(emu: &mut Emu) {
    sockets::create_socket(emu, "WSASocketA", true);
}

pub fn WSASocketW(emu: &mut Emu) {
    sockets::create_socket(emu, "WSASocketW", true);
}
