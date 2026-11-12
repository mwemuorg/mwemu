use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATAdminResolveCatalogPath(emu: &mut emu::Emu) {
    let h_cat_admin = WinApiAbi::arg(emu, 0);
    let pwsz_catalog_file = WinApiAbi::arg(emu, 1);
    let p_resolve_info = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptCATAdminResolveCatalogPath h_cat_admin:0x{:x} pwsz_catalog_file:0x{:x} p_resolve_info:0x{:x}",
        h_cat_admin,
        pwsz_catalog_file,
        p_resolve_info
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
