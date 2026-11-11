use super::DNS_ERROR_RCODE_SERVER_FAILURE;
use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn DnsQuery_A(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let wtype = arg(emu, 1);
    let opt = arg(emu, 2);

    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "dnsapi!DnsQuery_A name:'{}' type:{} options:0x{:x}",
        name,
        wtype,
        opt
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
    stdcall_ret(emu, 6);
}

pub(super) fn DnsQuery_W(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let wtype = arg(emu, 1);
    let opt = arg(emu, 2);

    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "dnsapi!DnsQuery_W name:'{}' type:{} options:0x{:x}",
        name,
        wtype,
        opt
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
    stdcall_ret(emu, 6);
}
