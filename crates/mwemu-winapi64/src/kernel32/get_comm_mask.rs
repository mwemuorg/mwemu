use crate::emu;

pub fn GetCommMask(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let mask_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetCommMask h:0x{:x} ptr:0x{:x}",
        h_file,
        mask_ptr
    );
    if mask_ptr != 0 {
        emu.maps.write_dword(mask_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
