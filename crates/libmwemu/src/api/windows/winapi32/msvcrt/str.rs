use crate::emu;

pub fn str(emu: &mut emu::Emu) {
    let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = emu.maps.read_string(s_ptr);
    log_red!(emu, "msvcrt!str 0x{:x} '{}'", s_ptr, s);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = s_ptr;
}
