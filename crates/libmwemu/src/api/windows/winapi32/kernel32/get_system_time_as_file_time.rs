use crate::emu;

pub fn GetSystemTimeAsFileTime(emu: &mut emu::Emu) {
    let ft_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    // Base: 2024-01-01 00:00:00 UTC, advance by emu.tick milliseconds
    let base_ft: u64 = 0x01DA_5E3E_A46C_0000;
    let filetime = base_ft + (emu.tick as u64) * 10_000;

    if ft_ptr != 0 {
        emu.maps
            .write_dword(ft_ptr, (filetime & 0xFFFF_FFFF) as u32);
        emu.maps.write_dword(ft_ptr + 4, (filetime >> 32) as u32);
    }

    log_red!(emu, "kernel32!GetSystemTimeAsFileTime ptr:0x{:x}", ft_ptr);

    emu.stack_pop32(false);
}
