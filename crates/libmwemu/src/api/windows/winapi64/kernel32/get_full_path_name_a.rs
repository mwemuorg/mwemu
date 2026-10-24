use crate::emu;

pub fn GetFullPathNameA(emu: &mut emu::Emu) {
    let file_ptr = emu.regs().rcx;
    let size = emu.regs().rdx;
    let buff = emu.regs().r8;
    let path = emu.regs().r9;

    let filename = emu.maps.read_string(file_ptr);
    log_red!(emu, "kernel32!GetFullPathNameA file: {}", filename);
    log::warn!("GetFullPathNameA: stub, returning 0");
    emu.regs_mut().rax = 0;
}
