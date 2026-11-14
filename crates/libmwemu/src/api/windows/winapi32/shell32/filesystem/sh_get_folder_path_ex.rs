use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetFolderPathEx(emu: &mut emu::Emu) {
    let rfid = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1) as u32;
    let h_token = WinApiAbi::arg(emu, 2) as u32;
    let psz_path = WinApiAbi::arg(emu, 3);
    let cch_path = WinApiAbi::arg(emu, 4) as u32;
    if psz_path != 0 && cch_path > 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\mwemu_user");
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathEx rfid:0x{:x} flags:0x{:x} => C:\\Users\\mwemu_user",
        rfid,
        dw_flags
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
