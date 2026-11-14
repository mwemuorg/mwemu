use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegisterTypeLib(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "oleaut32!RegisterTypeLib a0:0x{:x}", _a0);
    let _h = helper::handler_create("oleaut32://RegisterTypeLib");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
