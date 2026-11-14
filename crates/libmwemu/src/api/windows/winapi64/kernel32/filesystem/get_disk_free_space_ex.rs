use crate::emu;
use crate::winapi::common::strings;

pub fn GetDiskFreeSpaceExA(emu: &mut emu::Emu) {
    get_disk_free_space_ex(emu, "GetDiskFreeSpaceExA", false);
}

fn get_disk_free_space_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path = strings::read_text(emu, emu.regs().rcx, wide);
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
    log_red!(emu, "kernel32!{api} '{}'", path);
    emu.regs_mut().rax = 1;
}

pub fn GetDiskFreeSpaceExW(emu: &mut emu::Emu) {
    get_disk_free_space_ex(emu, "GetDiskFreeSpaceExW", true);
}
