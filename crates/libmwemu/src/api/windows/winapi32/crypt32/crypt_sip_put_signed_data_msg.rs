use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSIPPutSignedDataMsg(emu: &mut emu::Emu) {
    let p_subject_info = WinApiAbi::arg(emu, 0);
    let dw_encoding_type = WinApiAbi::arg(emu, 1);
    let pdw_index = WinApiAbi::arg(emu, 2);
    let cb_signed_data_msg = WinApiAbi::arg(emu, 3);
    let pb_signed_data_msg = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptSIPPutSignedDataMsg 0x{:x}",
        p_subject_info
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
