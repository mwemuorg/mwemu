use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn VarUI4FromBool(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    let _a1 = arg(emu, 1);
    log_red!(emu, "oleaut32!VarUI4FromBool a0:0x{:x} a1:0x{:x}", _a0, _a1);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
