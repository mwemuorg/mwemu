use crate::emu;

pub fn SHGetPathFromIDListW(emu: &mut emu::Emu) {
    let pidl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "shell32!SHGetPathFromIDListW pidl:0x{:x} path:0x{:x}",
        pidl,
        psz_path
    );
    if psz_path != 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
