use crate::emu;

pub fn PFXIsPFXBlob(emu: &mut emu::Emu) {
    let blob = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!PFXIsPFXBlob blob:0x{:x}", blob);
    emu.set_last_error(0);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    emu.regs_mut().rax = 1;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
