use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn ProcessSocketNotifications(emu: &mut emu::Emu) {
    let completion_port = WinApiAbi::arg(emu, 0) as u32;
    let count = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "ws2_32!ProcessSocketNotifications port:0x{:x} count:{}",
        completion_port,
        count
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 7);
}
