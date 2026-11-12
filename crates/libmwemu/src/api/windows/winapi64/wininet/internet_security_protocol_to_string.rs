use crate::emu;
use crate::winapi::common::strings;

pub fn InternetSecurityProtocolToStringA(emu: &mut emu::Emu) {
    internet_security_protocol_to_string(emu, "InternetSecurityProtocolToStringA", false);
}

fn internet_security_protocol_to_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let protocol = emu.regs().rcx as u32;
    let buf_ptr = emu.regs().rdx;
    let buf_len_ptr = emu.regs().r8;
    log_red!(emu, "wininet!{api} protocol:{}", protocol);
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = strings::read_text(emu, src, wide);
    }
    if buf_ptr != 0 {
        strings::write_string(emu, buf_ptr, "TLS 1.2", wide);
    }
    emu.regs_mut().rax = 1;
}

pub fn InternetSecurityProtocolToStringW(emu: &mut emu::Emu) {
    internet_security_protocol_to_string(emu, "InternetSecurityProtocolToStringW", true);
}
