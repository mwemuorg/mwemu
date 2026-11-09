use crate::emu;

pub fn getservbyport(emu: &mut emu::Emu) {
    let port = emu.regs().rcx as u32;
    let proto_ptr = emu.regs().rdx;
    let proto = if proto_ptr != 0 {
        emu.maps.read_string(proto_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!getservbyport port:{} proto:'{}'", port, proto);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
