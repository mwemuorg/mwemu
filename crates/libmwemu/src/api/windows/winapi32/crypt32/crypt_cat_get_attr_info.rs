use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATGetAttrInfo(emu: &mut emu::Emu) {
    let h_catalog = WinApiAbi::arg(emu, 0);
    let p_cat_member = WinApiAbi::arg(emu, 1);
    let pwsz_ref_tag = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptCATGetAttrInfo h_catalog:0x{:x} p_cat_member:0x{:x} pwsz_ref_tag:0x{:x}",
        h_catalog,
        p_cat_member,
        pwsz_ref_tag
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
