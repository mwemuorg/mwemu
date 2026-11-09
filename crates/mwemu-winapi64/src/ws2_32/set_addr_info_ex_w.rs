use crate::emu;

pub fn SetAddrInfoExW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let service_ptr = emu.regs().rdx;
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!SetAddrInfoExW name:'{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
