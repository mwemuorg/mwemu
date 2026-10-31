use crate::emu;

pub fn GetDiskFreeSpaceExW(emu: &mut emu::Emu) {
    let path = emu.maps.read_wide_string(emu.regs().rcx);
    let free_caller = emu.regs().rdx;
    let total = emu.regs().r8;
    let free_total = emu.regs().r9;
    if free_caller != 0 {
        emu.maps.write_qword(free_caller, 50_000_000_000);
    }
    if total != 0 {
        emu.maps.write_qword(total, 500_000_000_000);
    }
    if free_total != 0 {
        emu.maps.write_qword(free_total, 50_000_000_000);
    }
    log_red!(emu, "kernel32!GetDiskFreeSpaceExW '{}'", path);
    emu.regs_mut().rax = 1;
}
