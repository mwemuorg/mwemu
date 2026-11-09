use crate::emu;

pub fn SHGetPathFromIDListW(emu: &mut emu::Emu) {
    let pidl = emu.regs().rcx;
    let psz_path = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHGetPathFromIDListW pidl:0x{:x} path:0x{:x}",
        pidl,
        psz_path
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if psz_path != 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}
