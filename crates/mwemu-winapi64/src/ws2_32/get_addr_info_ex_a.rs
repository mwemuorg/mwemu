use crate::emu;

pub fn GetAddrInfoExA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let service_ptr = emu.regs().rdx;
    let namespace = emu.regs().r8 as u32;
    let hints_ptr = emu.regs().r9;
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
}
