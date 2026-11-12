use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SHGetFolderPathEx(emu: &mut emu::Emu) {
    let rfid = emu.regs().rcx;
    let dw_flags = emu.regs().rdx;
    let htoken = emu.regs().r8;
    let psz_path = emu.regs().r9;
    let cch_path = WinApiAbi::arg(emu, 5) as u32;
    let path = "C:\\Users\\user";
    if psz_path != 0 {
        for (i, ch) in path.encode_utf16().enumerate() {
            emu.maps.write_word(psz_path + (i as u64) * 2, ch);
        }
        emu.maps.write_word(psz_path + (path.len() as u64) * 2, 0);
    }
    log_red!(emu, "shell32!SHGetFolderPathEx -> '{}'", path);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
