use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn DnsNameCompare_A(emu: &mut emu::Emu) {
    dns_name_compare_(emu, "DnsNameCompare_A", false);
}

fn dns_name_compare_(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name1 = arg(emu, 0) as u64;
    let name2 = arg(emu, 1) as u64;
    let s1 = if name1 != 0 {
        strings::read_text(emu, name1, wide)
    } else {
        String::new()
    };
    let s2 = if name2 != 0 {
        strings::read_text(emu, name2, wide)
    } else {
        String::new()
    };
    log_red!(emu, "dnsapi!{api} '{}' vs '{}'", s1, s2);
    emu.regs_mut().rax = if s1.eq_ignore_ascii_case(&s2) { 1 } else { 0 };
    stdcall_ret(emu, 2);
}

pub fn DnsNameCompare_W(emu: &mut emu::Emu) {
    dns_name_compare_(emu, "DnsNameCompare_W", true);
}
