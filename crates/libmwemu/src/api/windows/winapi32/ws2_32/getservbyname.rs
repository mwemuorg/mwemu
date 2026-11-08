use crate::emu;

pub fn getservbyname(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let proto_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
