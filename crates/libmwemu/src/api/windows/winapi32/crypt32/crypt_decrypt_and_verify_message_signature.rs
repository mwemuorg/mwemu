use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptDecryptAndVerifyMessageSignature(emu: &mut emu::Emu) {
    let p_decrypt_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_verify_para = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_signer_index = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pb_encrypted_blob = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let cb_encrypted_blob = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let ppb_decrypted = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptDecryptAndVerifyMessageSignature 0x{:x}",
        p_decrypt_para
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}
