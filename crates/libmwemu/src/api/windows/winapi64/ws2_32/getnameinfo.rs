use crate::emu;

pub fn getnameinfo(emu: &mut emu::Emu) {
    let sa = emu.regs().rcx;
    let salen = emu.regs().rdx as u32;
    let host = emu.regs().r8;
    let hostlen = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!getnameinfo sa:0x{:x} host:0x{:x} hostlen:{}",
        sa,
        host,
        hostlen
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if host != 0 && hostlen > 9 {
        emu.maps.write_string(host, "localhost");
    }
    emu.regs_mut().rax = 0;
}
