use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateErrorInfo(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "oleaut32!CreateErrorInfo a0:0x{:x}", _a0);
    let _h = helper::handler_create("oleaut32://CreateErrorInfo");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
