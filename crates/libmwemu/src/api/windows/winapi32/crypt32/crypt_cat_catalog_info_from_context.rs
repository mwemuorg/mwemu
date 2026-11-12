use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATCatalogInfoFromContext(emu: &mut emu::Emu) {
    let h_cat_info = WinApiAbi::arg(emu, 0);
    let p_cat_info = WinApiAbi::arg(emu, 1);
    let dw_flags = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptCATCatalogInfoFromContext h_cat_info:0x{:x} p_cat_info:0x{:x} dw_flags:0x{:x}",
        h_cat_info,
        p_cat_info,
        dw_flags
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
