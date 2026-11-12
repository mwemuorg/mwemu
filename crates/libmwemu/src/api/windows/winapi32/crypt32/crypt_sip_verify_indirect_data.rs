use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptSIPVerifyIndirectData(emu: &mut emu::Emu) {
    let p_subject_info = WinApiAbi::arg(emu, 0);
    let p_indirect_data = WinApiAbi::arg(emu, 1);
    let dw_reserved = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptSIPVerifyIndirectData p_subject_info:0x{:x} p_indirect_data:0x{:x} dw_reserved:0x{:x}",
        p_subject_info,
        p_indirect_data,
        dw_reserved
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
