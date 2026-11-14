use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptCATCDFEnumMembersByCDFTagEx(emu: &mut emu::Emu) {
    let p_cdf = WinApiAbi::arg(emu, 0);
    let pwsz_prev_tag = WinApiAbi::arg(emu, 1);
    let pfn_parse_error = WinApiAbi::arg(emu, 2);
    let pp_member = WinApiAbi::arg(emu, 3);
    let f_continue_on_error = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumMembersByCDFTagEx 0x{:x}",
        p_cdf
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
