use crate::emu;

pub fn getenv(emu: &mut emu::Emu, api: &str) {
    let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}' => NULL", api, name);
    emu.regs_mut().rax = 0;
}
