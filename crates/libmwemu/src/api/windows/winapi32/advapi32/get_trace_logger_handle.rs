use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTraceLoggerHandle(emu: &mut emu::Emu) {
    let handle = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "advapi32!GetTraceLoggerHandle h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    stdcall_ret(emu, 1);
    emu.set_last_error(0);
    WinApiAbi::set_ret64(emu, 1);
}
