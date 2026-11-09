use crate::emu;

pub fn CryptRetrieveObjectByUrl(emu: &mut emu::Emu) {
    let psz_url = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let psz_object_oid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_retrieve_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_timeout = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let ppv_object = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let h_async_retrieve = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let p_credentials = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptRetrieveObjectByUrl 0x{:x}", psz_url);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
