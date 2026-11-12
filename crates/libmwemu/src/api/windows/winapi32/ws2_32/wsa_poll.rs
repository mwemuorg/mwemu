use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn WSAPoll(emu: &mut emu::Emu) {
    let fds_ptr = WinApiAbi::arg(emu, 0);
    let nfds = WinApiAbi::arg(emu, 1) as u32;
    let timeout = WinApiAbi::arg(emu, 2) as i32;
    log_red!(emu, "ws2_32!WSAPoll nfds:{} timeout:{}", nfds, timeout);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
