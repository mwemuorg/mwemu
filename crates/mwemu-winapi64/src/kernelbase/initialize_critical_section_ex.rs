use crate::emu;

pub fn InitializeCriticalSectionEx(emu: &mut emu::Emu) {
    let cs_ptr = emu.regs().rcx;
    let spin_count = emu.regs().rdx;
    let flags = emu.regs().r8;
    if cs_ptr != 0 {
        emu.maps.write_qword(cs_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!InitializeCriticalSectionEx ptr:0x{:x} spin:{} flags:0x{:x}",
        cs_ptr,
        spin_count,
        flags
    );
    emu.regs_mut().rax = 1;
}
