use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptDecryptMessage(emu: &mut emu::Emu) {
    let p_decrypt_para = WinApiAbi::arg(emu, 0);
    let pb_encrypted_blob = WinApiAbi::arg(emu, 1);
    let cb_encrypted_blob = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptDecryptMessage p_decrypt_para:0x{:x} pb_encrypted_blob:0x{:x} cb_encrypted_blob:0x{:x}",
        p_decrypt_para,
        pb_encrypted_blob,
        cb_encrypted_blob
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
