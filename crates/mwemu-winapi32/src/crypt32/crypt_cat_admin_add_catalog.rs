use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminAddCatalog(emu: &mut emu::Emu) {
    let h_cat_admin = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pwsz_catalog_file = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pwsz_select_base_name = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptCATAdminAddCatalog 0x{:x}", h_cat_admin);
    let hndl = helper::handler_create("CryptCATAdminAddCatalog");
    emu.regs_mut().rax = hndl;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
