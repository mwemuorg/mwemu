use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn WSAWaitForMultipleEvents(emu: &mut emu::Emu) {
    let count = WinApiAbi::arg(emu, 0) as u32;
    let events_ptr = WinApiAbi::arg(emu, 1);
    let wait_all = WinApiAbi::arg(emu, 2) as u32;
    let timeout = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "ws2_32!WSAWaitForMultipleEvents count:{} wait_all:{} timeout:{}",
        count,
        wait_all,
        timeout
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}
