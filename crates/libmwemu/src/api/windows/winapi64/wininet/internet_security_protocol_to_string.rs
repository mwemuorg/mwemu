use crate::emu;

pub fn InternetSecurityProtocolToStringA(emu: &mut emu::Emu) {
    let protocol = emu.regs().rcx as u32;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(
        emu,
        "wininet!InternetSecurityProtocolToStringA protocol:{}",
        protocol
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    if buf_ptr != 0 {
        emu.maps.write_string(buf_ptr, "TLS 1.2");
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetSecurityProtocolToStringW(emu: &mut emu::Emu) {
    let protocol = emu.regs().rcx as u32;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(
        emu,
        "wininet!InternetSecurityProtocolToStringW protocol:{}",
        protocol
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    if buf_ptr != 0 {
        emu.maps.write_string(buf_ptr, "TLS 1.2");
    }
    emu.regs_mut().rax = 1;
}
