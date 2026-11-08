use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn SysFreeString(emu: &mut emu::Emu) {
    let str_ptr = arg(emu, 0) as u64;

    log_red!(emu, "oleaut32!SysFreeString 0x{:x}", str_ptr);

    stdcall_ret(emu, 1);
}
