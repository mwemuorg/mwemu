use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATAdminEnumCatalogFromHash(emu: &mut emu::Emu) {
    let h_cat_admin = WinApiAbi::arg(emu, 0);
    let pb_hash = WinApiAbi::arg(emu, 1);
    let cb_hash = WinApiAbi::arg(emu, 2);
    let dw_flags = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "crypt32!CryptCATAdminEnumCatalogFromHash 0x{:x}",
        h_cat_admin
    );
    let hndl = helper::handler_create("CryptCATAdminEnumCatalogFromHash");
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 5);
}
