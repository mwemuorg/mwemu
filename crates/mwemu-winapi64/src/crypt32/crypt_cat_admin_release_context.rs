use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminReleaseContext(emu: &mut emu::Emu) {
    let h_cat_admin = emu.regs().rcx;
    let dw_flags = emu.regs().rdx as u32;
    log_red!(
        emu,
        "crypt32!CryptCATAdminReleaseContext admin:0x{:x} flags:0x{:x}",
        h_cat_admin,
        dw_flags
    );
    if h_cat_admin != 0 {
        helper::handler_close(h_cat_admin);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
