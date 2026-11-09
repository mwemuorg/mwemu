use crate::emu;

pub fn CryptGetObjectUrl(emu: &mut emu::Emu) {
    let psz_url_oid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pv_para = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p_url_array = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pcb_url_array = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let p_url_info = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let pcb_url_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    let pv_reserved = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptGetObjectUrl 0x{:x}", psz_url_oid);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
