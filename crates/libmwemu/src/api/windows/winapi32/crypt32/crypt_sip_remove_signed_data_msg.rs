use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptSIPRemoveSignedDataMsg(emu: &mut emu::Emu) {
    let p_subject_info = WinApiAbi::arg(emu, 0);
    let dw_index = WinApiAbi::arg(emu, 1);
    let dw_reserved = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptSIPRemoveSignedDataMsg p_subject_info:0x{:x} dw_index:0x{:x} dw_reserved:0x{:x}",
        p_subject_info,
        dw_index,
        dw_reserved
    );
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
