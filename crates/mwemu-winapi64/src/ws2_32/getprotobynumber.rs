use crate::emu;

pub fn getprotobynumber(emu: &mut emu::Emu) {
    let proto = emu.regs().rcx as u32;
    log_red!(emu, "ws2_32!getprotobynumber proto:{}", proto);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
