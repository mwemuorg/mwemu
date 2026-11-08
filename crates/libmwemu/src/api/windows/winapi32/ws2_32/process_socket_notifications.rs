use crate::emu;

pub fn ProcessSocketNotifications(emu: &mut emu::Emu) {
    let completion_port = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let count = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "ws2_32!ProcessSocketNotifications port:0x{:x} count:{}",
        completion_port,
        count
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
