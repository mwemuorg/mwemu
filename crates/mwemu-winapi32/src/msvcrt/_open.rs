use crate::emu;
use crate::winapi::helper;

pub fn _open(emu: &mut emu::Emu, api: &str) {
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let path = emu.maps.read_string(path_ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, path);
    emu.regs_mut().rax = helper::handler_create(&path);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
