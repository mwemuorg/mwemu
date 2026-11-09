use crate::emu;
use crate::winapi::helper;

pub fn WSAWaitForMultipleEvents(emu: &mut emu::Emu) {
    let count = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let events_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let wait_all = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let timeout = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!WSAWaitForMultipleEvents count:{} wait_all:{} timeout:{}",
        count,
        wait_all,
        timeout
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
