use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptBinaryToStringA(emu: &mut emu::Emu) {
    crypt_binary_to_string(emu, "CryptBinaryToStringA", false);
}

pub(super) fn CryptBinaryToStringW(emu: &mut emu::Emu) {
    crypt_binary_to_string(emu, "CryptBinaryToStringW", true);
}

fn crypt_binary_to_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let data_ptr = emu.regs().rcx;
    let data_len = emu.regs().rdx;
    let flags = emu.regs().r8;
    let str_ptr = emu.regs().r9;
    let str_len_ptr = WinApiAbi::arg(emu, 4);
    if wide {
        emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_binary_to_string_w(
            emu,
            data_ptr,
            data_len,
            flags,
            str_ptr,
            str_len_ptr,
        );
    } else {
        emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_binary_to_string_a(
            emu,
            data_ptr,
            data_len,
            flags,
            str_ptr,
            str_len_ptr,
        );
    }
}
