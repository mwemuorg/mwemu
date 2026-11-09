use crate::emu;

pub fn DnsNameCompare_A(emu: &mut emu::Emu) {
    let name1 = emu.regs().rcx;
    let name2 = emu.regs().rdx;
    let s1 = if name1 != 0 {
        emu.maps.read_string(name1)
    } else {
        String::new()
    };
    let s2 = if name2 != 0 {
        emu.maps.read_string(name2)
    } else {
        String::new()
    };
    log_red!(emu, "dnsapi!DnsNameCompare_A '{}' vs '{}'", s1, s2);
    emu.regs_mut().rax = if s1.eq_ignore_ascii_case(&s2) { 1 } else { 0 };
}
