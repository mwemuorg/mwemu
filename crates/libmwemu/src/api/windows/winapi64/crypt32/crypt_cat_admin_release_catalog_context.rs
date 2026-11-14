use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminReleaseCatalogContext(emu: &mut emu::Emu) {
    let h_cat_admin = emu.regs().rcx;
    let h_cat_info = emu.regs().rdx;
    let dw_flags = emu.regs().r8 as u32;
    log_red!(
        emu,
        "crypt32!CryptCATAdminReleaseCatalogContext admin:0x{:x} info:0x{:x} flags:0x{:x}",
        h_cat_admin,
        h_cat_info,
        dw_flags
    );
    if h_cat_info != 0 {
        helper::handler_close(h_cat_info);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
