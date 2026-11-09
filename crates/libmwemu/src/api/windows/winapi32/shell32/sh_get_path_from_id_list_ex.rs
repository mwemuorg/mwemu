use crate::emu;

pub fn SHGetPathFromIDListEx(emu: &mut emu::Emu) {
    let pidl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cch_path = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "shell32!SHGetPathFromIDListEx pidl:0x{:x} path:0x{:x} cch:{}",
        pidl,
        psz_path,
        cch_path
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    if psz_path != 0 && cch_path > 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
