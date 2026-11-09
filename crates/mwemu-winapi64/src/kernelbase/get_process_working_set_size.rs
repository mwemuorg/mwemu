use crate::emu;

pub fn GetProcessWorkingSetSize(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let min_ptr = emu.regs().rdx;
    let max_ptr = emu.regs().r8;
    if min_ptr != 0 {
        emu.maps.write_qword(min_ptr, 204800); // 200 KB min
    }
    if max_ptr != 0 {
        emu.maps.write_qword(max_ptr, 1413120); // ~1.3 MB max
    }
    log_red!(emu, "kernelbase!GetProcessWorkingSetSize h:0x{:x}", handle);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
