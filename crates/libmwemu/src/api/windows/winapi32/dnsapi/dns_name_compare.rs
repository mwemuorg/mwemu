use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DnsNameCompare_A(emu: &mut emu::Emu) {
    let name1 = arg(emu, 0) as u64;
    let name2 = arg(emu, 1) as u64;
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
    stdcall_ret(emu, 2);
}

pub fn DnsNameCompare_W(emu: &mut emu::Emu) {
    let name1 = arg(emu, 0) as u64;
    let name2 = arg(emu, 1) as u64;
    let s1 = if name1 != 0 {
        emu.maps.read_wide_string(name1)
    } else {
        String::new()
    };
    let s2 = if name2 != 0 {
        emu.maps.read_wide_string(name2)
    } else {
        String::new()
    };
    log_red!(emu, "dnsapi!DnsNameCompare_W '{}' vs '{}'", s1, s2);
    emu.regs_mut().rax = if s1.eq_ignore_ascii_case(&s2) { 1 } else { 0 };
    stdcall_ret(emu, 2);
}
