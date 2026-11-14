use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptSignMessage(emu: &mut emu::Emu) {
    let p_sign_para = WinApiAbi::arg(emu, 0);
    let f_detached_signature = WinApiAbi::arg(emu, 1);
    let c_to_be_signed = WinApiAbi::arg(emu, 2);
    let rgb_pb_to_be_signed = WinApiAbi::arg(emu, 3);
    let rgcb_to_be_signed = WinApiAbi::arg(emu, 4);
    log_red!(emu, "crypt32!CryptSignMessage 0x{:x}", p_sign_para);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
