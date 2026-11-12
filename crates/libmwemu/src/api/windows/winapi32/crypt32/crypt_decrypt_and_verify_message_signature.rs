use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptDecryptAndVerifyMessageSignature(emu: &mut emu::Emu) {
    let p_decrypt_para = WinApiAbi::arg(emu, 0);
    let p_verify_para = WinApiAbi::arg(emu, 1);
    let dw_signer_index = WinApiAbi::arg(emu, 2);
    let pb_encrypted_blob = WinApiAbi::arg(emu, 3);
    let cb_encrypted_blob = WinApiAbi::arg(emu, 4);
    let ppb_decrypted = WinApiAbi::arg(emu, 5);
    log_red!(
        emu,
        "crypt32!CryptDecryptAndVerifyMessageSignature 0x{:x}",
        p_decrypt_para
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}
