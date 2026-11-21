use crate::emu::Emu;
use crate::winapi::policy;

pub fn WSASend(emu: &mut Emu) {
    policy::unsupported(emu, "ws2_32", "WSASend", emu.pc());
}

pub fn WSARecv(emu: &mut Emu) {
    policy::unsupported(emu, "ws2_32", "WSARecv", emu.pc());
}

pub fn WSASendTo(emu: &mut Emu) {
    policy::unsupported(emu, "ws2_32", "WSASendTo", emu.pc());
}

pub fn WSARecvFrom(emu: &mut Emu) {
    policy::unsupported(emu, "ws2_32", "WSARecvFrom", emu.pc());
}

pub fn WSASendMsg(emu: &mut Emu) {
    policy::unsupported(emu, "ws2_32", "WSASendMsg", emu.pc());
}

pub fn WSAIoctl(emu: &mut Emu) {
    policy::unsupported(emu, "ws2_32", "WSAIoctl", emu.pc());
}
