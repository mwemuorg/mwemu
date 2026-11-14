use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn VarFix(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "oleaut32!VarFix a0:0x{:x}", _a0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
