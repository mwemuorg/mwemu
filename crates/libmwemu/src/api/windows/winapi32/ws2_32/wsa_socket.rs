use crate::emu::Emu;
use crate::winapi::common::winsock;

pub fn WSASocketA(emu: &mut Emu) {
    winsock::socket(emu, "WSASocketA");
}

pub fn WSASocketW(emu: &mut Emu) {
    winsock::socket(emu, "WSASocketW");
}
