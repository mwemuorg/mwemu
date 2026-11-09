use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn ntohs(emu: &mut emu::Emu) {
    let val = arg(emu, 0) as u16;
    log_red!(emu, "ws2_32!ntohs {}", val);
    emu.regs_mut().rax = u16::from_be(val) as u64;
    stdcall_ret(emu, 1);
}
