use crate::emu;

pub fn ntohs(emu: &mut emu::Emu) {
    let val = emu.regs().rcx as u16;
    log_red!(emu, "ws2_32!ntohs val:0x{:x}", val);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = u16::from_be(val) as u64;
}
