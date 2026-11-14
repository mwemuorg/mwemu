use crate::emu;

pub(super) fn crt_controlfp_s(emu: &mut emu::Emu) {
    let current = emu.regs().rcx;
    let new_val = emu.regs().rdx;
    let mask = emu.regs().r8;
    if current != 0 {
        emu.maps.write_dword(current, 0x0008_001f);
    }
    log_red!(emu, "wincrt!_controlfp_s 0x{:x} 0x{:x}", new_val, mask);
    emu.regs_mut().rax = 0;
}
