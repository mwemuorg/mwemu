use super::DNS_ERROR_RCODE_SERVER_FAILURE;
use crate::emu;
use crate::winapi::common::strings;

pub(super) fn DnsQuery_A(emu: &mut emu::Emu) {
    dns_query_(emu, "DnsQuery_A", false);
}

fn dns_query_(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = emu.regs().rcx;
    let wtype = emu.regs().rdx;
    let options = emu.regs().r8;

    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "dnsapi!{api} name:'{}' type:{} options:0x{:x}",
        name,
        wtype,
        options
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
}

pub(super) fn DnsQuery_W(emu: &mut emu::Emu) {
    dns_query_(emu, "DnsQuery_W", true);
}
