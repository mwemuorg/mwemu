use crate::emu;

pub fn ProcessSocketNotifications(emu: &mut emu::Emu) {
    let completion_port = emu.regs().rcx;
    let count = emu.regs().rdx;
    log_red!(
        emu,
        "ws2_32!ProcessSocketNotifications port:0x{:x} count:{}",
        completion_port,
        count
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
