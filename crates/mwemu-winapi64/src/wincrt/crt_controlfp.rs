use crate::emu;

pub(super) fn crt_controlfp(emu: &mut emu::Emu) {
    let new_val = emu.regs().rcx;
    let mask = emu.regs().rdx;
    log_red!(emu, "wincrt!_controlfp 0x{:x} 0x{:x}", new_val, mask);
    emu.regs_mut().rax = 0x0008_001f;
}
