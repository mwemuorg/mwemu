use crate::emu;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants;

pub(super) fn OleInitialize(emu: &mut emu::Emu) {
    log_red!(emu, "ole32!OleInitialize");
    emu.regs_mut().rax = constants::S_OK;
    stdcall_ret(emu, 1);
}
