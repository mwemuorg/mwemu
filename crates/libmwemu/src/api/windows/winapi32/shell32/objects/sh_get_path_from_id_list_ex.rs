use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetPathFromIDListEx(emu: &mut emu::Emu) {
    let pidl = WinApiAbi::arg(emu, 0);
    let psz_path = WinApiAbi::arg(emu, 1);
    let cch_path = WinApiAbi::arg(emu, 2) as u32;
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
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
