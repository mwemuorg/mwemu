use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptSignMessageWithKey(emu: &mut emu::Emu) {
    let p_sign_para = WinApiAbi::arg(emu, 0);
    let pb_to_be_signed = WinApiAbi::arg(emu, 1);
    let cb_to_be_signed = WinApiAbi::arg(emu, 2);
    let ppb_signed_blob = WinApiAbi::arg(emu, 3);
    log_red!(emu, "crypt32!CryptSignMessageWithKey 0x{:x}", p_sign_para);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
