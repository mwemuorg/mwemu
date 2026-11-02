use crate::emu;

pub fn GetDiskFreeSpaceExW(emu: &mut emu::Emu) {
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let free_caller = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let total = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let free_total = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;

    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
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

    log_red!(emu, "kernel32!GetDiskFreeSpaceExW '{}'", path);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
