use crate::emu;

pub fn GetProcessWorkingSetSize(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let min_ptr = emu.regs().rdx;
    let max_ptr = emu.regs().r8;
    log_red!(emu, "kernel32!GetProcessWorkingSetSize h:0x{:x}", handle);
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 0x20000);
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 0x1400000);
    }
    emu.regs_mut().rax = 1;
}
