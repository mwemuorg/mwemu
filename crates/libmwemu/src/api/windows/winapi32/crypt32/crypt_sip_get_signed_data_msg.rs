use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSIPGetSignedDataMsg(emu: &mut emu::Emu) {
    let p_subject_info = WinApiAbi::arg(emu, 0);
    let pdw_encoding_type = WinApiAbi::arg(emu, 1);
    let dw_index = WinApiAbi::arg(emu, 2);
    let pcb_signed_data_msg = WinApiAbi::arg(emu, 3);
    let pb_signed_data_msg = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptSIPGetSignedDataMsg 0x{:x}",
        p_subject_info
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
