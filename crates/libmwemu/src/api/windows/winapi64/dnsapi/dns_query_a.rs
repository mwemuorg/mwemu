use super::DNS_ERROR_RCODE_SERVER_FAILURE;
use crate::emu;

pub(super) fn DnsQuery_A(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let wtype = emu.regs().rdx;
    let options = emu.regs().r8;

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
        options
    );

    emu.regs_mut().rax = DNS_ERROR_RCODE_SERVER_FAILURE;
}
