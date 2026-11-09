use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegisterTypeLibForUser(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "oleaut32!RegisterTypeLibForUser a0:0x{:x}", _a0);
    let _h = helper::handler_create("oleaut32://RegisterTypeLibForUser");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
