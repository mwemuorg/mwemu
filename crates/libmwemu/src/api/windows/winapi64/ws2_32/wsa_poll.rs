use crate::emu;
use crate::winapi::helper;

pub fn WSAPoll(emu: &mut emu::Emu) {
    let fds_ptr = emu.regs().rcx;
    let nfds = emu.regs().rdx as u32;
    let timeout = emu.regs().r8 as i32;
    log_red!(emu, "ws2_32!WSAPoll nfds:{} timeout:{}", nfds, timeout);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
