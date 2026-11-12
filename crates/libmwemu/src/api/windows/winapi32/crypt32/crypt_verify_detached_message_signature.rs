use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptVerifyDetachedMessageSignature(emu: &mut emu::Emu) {
    let p_verify_para = WinApiAbi::arg(emu, 0);
    let dw_signer_index = WinApiAbi::arg(emu, 1);
    let pb_detached_sign_blob = WinApiAbi::arg(emu, 2);
    let cb_detached_sign_blob = WinApiAbi::arg(emu, 3);
    let c_to_be_signed = WinApiAbi::arg(emu, 4);
    let rgb_pb_to_be_signed = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "crypt32!CryptVerifyDetachedMessageSignature 0x{:x}",
        p_verify_para
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
