use crate::emu;

pub fn GetFullPathNameW(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let buff = emu.regs().r8;
    let path = emu.regs().r9;

    let filename = emu.maps.read_wide_string(file_ptr);
    log_red!(emu, "kernel32!GetFullPathNameW file: {}", filename);
    log::warn!("GetFullPathNameW: stub, returning 0");
    emu.regs_mut().rax = 0;
}
