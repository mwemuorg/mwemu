use crate::emu;

pub fn _localtime64(emu: &mut emu::Emu) {
    let time_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _time_val = if time_ptr != 0 {
        emu.maps.read_qword(time_ptr).unwrap_or(0)
    } else {
        0
    };
    let tm_addr = emu.maps.alloc(36).unwrap_or_default();
    if tm_addr != 0
        && emu
            .maps
            .create_map(
                "tm_struct",
                tm_addr,
                36,
                crate::maps::mem64::Permission::from_flags(true, true, false),
            )
            .is_ok()
    {
        emu.maps.write_dword(tm_addr, 0); // tm_sec
        emu.maps.write_dword(tm_addr + 4, 30); // tm_min
        emu.maps.write_dword(tm_addr + 8, 12); // tm_hour
        emu.maps.write_dword(tm_addr + 12, 1); // tm_mday
        emu.maps.write_dword(tm_addr + 16, 0); // tm_mon
        emu.maps.write_dword(tm_addr + 20, 126); // tm_year (2026)
        emu.maps.write_dword(tm_addr + 24, 4); // tm_wday
        emu.maps.write_dword(tm_addr + 28, 0); // tm_yday
        emu.maps.write_dword(tm_addr + 32, 0); // tm_isdst
    }
    log_red!(emu, "msvcrt!_localtime64 => 0x{:x}", tm_addr);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = tm_addr;
}
