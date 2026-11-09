use crate::emu;

pub fn _putenv(emu: &mut emu::Emu, api: &str) {
    let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = emu.maps.read_string(ptr);
    log_red!(emu, "msvcrt!{} '{}'", api, s);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
