use crate::emu;

pub fn getservbyname(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let proto_ptr = emu.regs().rdx;
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    let proto = if proto_ptr != 0 {
        emu.maps.read_string(proto_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!getservbyname name:'{}' proto:'{}'",
        name,
        proto
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
