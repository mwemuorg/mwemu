use crate::emu;

pub fn getprotobyname(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "ws2_32!getprotobyname name:'{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
