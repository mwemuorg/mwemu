use crate::emu;

pub fn SHGetFolderPathEx(emu: &mut emu::Emu) {
    let rfid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let h_token = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let psz_path = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let cch_path = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
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
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
