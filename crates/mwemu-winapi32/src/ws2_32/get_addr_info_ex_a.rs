use crate::emu;

pub fn GetAddrInfoExA(emu: &mut emu::Emu) {
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let service_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let namespace = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    let service = if service_ptr != 0 {
        emu.maps.read_string(service_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "ws2_32!GetAddrInfoExA name:'{}' service:'{}' ns:{}",
        name,
        service,
        namespace
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
