use crate::emu;

pub(super) fn crt_getenv(emu: &mut emu::Emu, api: &str) {
    let name_ptr = emu.regs().rcx;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "wincrt!{} '{}' => NULL", api, name);
    emu.regs_mut().rax = 0;
}
