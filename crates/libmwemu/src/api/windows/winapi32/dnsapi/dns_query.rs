use super::DNS_ERROR_RCODE_SERVER_FAILURE;
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn DnsQuery_A(emu: &mut emu::Emu) {
    dns_query_(emu, "DnsQuery_A", false);
}

fn dns_query_(emu: &mut emu::Emu, api: &str, wide: bool) {
    let name_ptr = arg(emu, 0) as u64;
    let wtype = arg(emu, 1);
    let opt = arg(emu, 2);

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
        opt
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
    stdcall_ret(emu, 6);
}

pub(super) fn DnsQuery_W(emu: &mut emu::Emu) {
    dns_query_(emu, "DnsQuery_W", true);
}
