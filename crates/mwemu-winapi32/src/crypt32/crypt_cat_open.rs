use crate::emu;
use crate::winapi::helper;

pub fn CryptCATOpen(emu: &mut emu::Emu) {
    let pwsz_file_name = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fdw_open_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let h_prov = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_public_version = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let dw_encoded_msg_type = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptCATOpen 0x{:x}", pwsz_file_name);
    let hndl = helper::handler_create("CryptCATOpen");
    emu.regs_mut().rax = hndl;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
