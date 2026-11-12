use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetDiskFreeSpaceExA(emu: &mut emu::Emu) {
    get_disk_free_space_ex(emu, "GetDiskFreeSpaceExA", false);
}

fn get_disk_free_space_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let path_ptr = WinApiAbi::arg(emu, 0);
    let free_caller = WinApiAbi::arg(emu, 1);
    let total = WinApiAbi::arg(emu, 2);
    let free_total = WinApiAbi::arg(emu, 3);

    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };

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

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}

pub fn GetDiskFreeSpaceExW(emu: &mut emu::Emu) {
    get_disk_free_space_ex(emu, "GetDiskFreeSpaceExW", true);
}
