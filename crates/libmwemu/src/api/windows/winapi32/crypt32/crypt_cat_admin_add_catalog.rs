use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATAdminAddCatalog(emu: &mut emu::Emu) {
    let h_cat_admin = WinApiAbi::arg(emu, 0);
    let pwsz_catalog_file = WinApiAbi::arg(emu, 1);
    let pwsz_select_base_name = WinApiAbi::arg(emu, 2);
    let dw_flags = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!CryptCATAdminAddCatalog 0x{:x}", h_cat_admin);
    let hndl = helper::handler_create("CryptCATAdminAddCatalog");
    emu.regs_mut().rax = hndl;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
