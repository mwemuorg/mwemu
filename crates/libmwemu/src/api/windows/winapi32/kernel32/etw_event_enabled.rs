use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn EtwEventEnabled(emu: &mut emu::Emu) {
    let reg_handle = WinApiAbi::arg(emu, 0);
    let event_descriptor = WinApiAbi::arg(emu, 1);
    let level = if event_descriptor != 0 {
        emu.maps.read_byte(event_descriptor).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "kernel32!EventEnabled handle:0x{:x} level:{}",
        reg_handle,
        level
    );
    stdcall_ret(emu, 3);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
