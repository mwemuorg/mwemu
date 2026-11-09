use crate::emu;

pub fn GetAddrInfoW(emu: &mut emu::Emu) {
    let node_ptr = emu.regs().rcx;
    let service_ptr = emu.regs().rdx;
    let hints_ptr = emu.regs().r8;
    let result_ptr_ptr = emu.regs().r9;
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
}
