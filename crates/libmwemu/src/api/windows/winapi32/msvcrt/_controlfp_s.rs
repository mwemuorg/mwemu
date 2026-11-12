use crate::emu;

pub fn _controlfp_s(emu: &mut emu::Emu) {
    let p_current = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if p_current != 0 {
        emu.maps.write_dword(p_current, 0x0008_001f);
    }
    log_red!(emu, "msvcrt!_controlfp_s");
    emu.regs_mut().rax = 0;
}
