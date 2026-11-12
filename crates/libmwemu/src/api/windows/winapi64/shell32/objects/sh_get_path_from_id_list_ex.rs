use crate::emu;

pub fn SHGetPathFromIDListEx(emu: &mut emu::Emu) {
    let pidl = emu.regs().rcx;
    let psz_path = emu.regs().rdx;
    let cch_path = emu.regs().r8;
    log_red!(
        emu,
        "shell32!SHGetPathFromIDListEx pidl:0x{:x} path:0x{:x} cch:{}",
        pidl,
        psz_path,
        cch_path
    );

    if psz_path != 0 && cch_path > 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}
