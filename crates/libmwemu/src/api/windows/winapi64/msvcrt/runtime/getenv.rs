use crate::emu;

pub fn getenv(emu: &mut emu::Emu, api: &str) {
    let ptr = emu.regs().rcx;
    let name = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
