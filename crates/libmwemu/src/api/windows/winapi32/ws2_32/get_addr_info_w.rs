use crate::emu;

pub fn GetAddrInfoW(emu: &mut emu::Emu) {
    let node_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let hints_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let result_ptr_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let node = if node_ptr != 0 {
        emu.maps.read_wide_string(node_ptr)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        emu.maps.read_wide_string(service_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!GetAddrInfoW node:'{}' service:'{}'",
        node,
        service
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
