use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATAdminReleaseCatalogContext(emu: &mut emu::Emu) {
    let h_cat_admin = WinApiAbi::arg(emu, 0);
    let h_cat_info = WinApiAbi::arg(emu, 1);
    let dw_flags = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptCATAdminReleaseCatalogContext h_cat_admin:0x{:x} h_cat_info:0x{:x} dw_flags:0x{:x}",
        h_cat_admin,
        h_cat_info,
        dw_flags
    );
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
