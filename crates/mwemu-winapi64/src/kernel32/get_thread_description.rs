use crate::emu;

pub fn GetThreadDescription(emu: &mut emu::Emu) {
    let h_thread = emu.regs().rcx;
    let desc_ptr_ptr = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!GetThreadDescription h:0x{:x} ptr:0x{:x}",
        h_thread,
        desc_ptr_ptr
    );
    if desc_ptr_ptr != 0 {
        emu.maps.write_qword(desc_ptr_ptr, 0);
    }
    emu.regs_mut().rax = 0;
}
