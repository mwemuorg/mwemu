use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn CryptStringToBinaryW(emu: &mut emu::Emu) {
    let string = arg(emu, 0) as u64;
    let num_chars = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let ptr = arg(emu, 3) as u64;
    let inout_sz = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_w(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
    stdcall_ret(emu, 7);
}
