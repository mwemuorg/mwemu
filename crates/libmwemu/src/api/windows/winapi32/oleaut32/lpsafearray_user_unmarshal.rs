use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn LPSAFEARRAY_UserUnmarshal(emu: &mut emu::Emu) {
    let _a0 = arg(emu, 0);
    log_red!(emu, "oleaut32!LPSAFEARRAY_UserUnmarshal a0:0x{:x}", _a0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
