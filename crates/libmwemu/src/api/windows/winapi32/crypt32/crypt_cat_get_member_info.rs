use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATGetMemberInfo(emu: &mut emu::Emu) {
    let h_catalog = WinApiAbi::arg(emu, 0);
    let pwsz_ref_tag = WinApiAbi::arg(emu, 1);
    let p_cat_member = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptCATGetMemberInfo h_catalog:0x{:x} pwsz_ref_tag:0x{:x} p_cat_member:0x{:x}",
        h_catalog,
        pwsz_ref_tag,
        p_cat_member
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
