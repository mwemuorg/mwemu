use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CryptBinaryToStringW(emu: &mut emu::Emu) {
    let data_ptr = arg(emu, 0) as u64;
    let data_len = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let str_ptr = arg(emu, 3) as u64;
    let str_len_ptr = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_binary_to_string_w(
        emu,
        data_ptr,
        data_len,
        flags,
        str_ptr,
        str_len_ptr,
    );
    stdcall_ret(emu, 5);
}
