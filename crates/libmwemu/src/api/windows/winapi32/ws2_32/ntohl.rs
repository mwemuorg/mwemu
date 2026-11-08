use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn ntohl(emu: &mut emu::Emu) {
    let val = arg(emu, 0);
    log_red!(emu, "ws2_32!ntohl 0x{:x}", val);
    emu.regs_mut().rax = u32::from_be(val) as u64;
    stdcall_ret(emu, 1);
}
