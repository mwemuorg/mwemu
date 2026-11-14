use crate::emu;
use crate::winapi::helper;

pub fn WSAWaitForMultipleEvents(emu: &mut emu::Emu) {
    let count = emu.regs().rcx as u32;
    let events_ptr = emu.regs().rdx;
    let wait_all = emu.regs().r8 as u32;
    let timeout = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!WSAWaitForMultipleEvents count:{} wait_all:{} timeout:{}",
        count,
        wait_all,
        timeout
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
