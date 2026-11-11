use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptStringToBinary(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}

pub fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    let string = arg(emu, 0) as u64;
    let num_chars = arg(emu, 1) as u64;
    let flags = arg(emu, 2) as u64;
    let ptr = arg(emu, 3) as u64;
    let inout_sz = arg(emu, 4) as u64;

    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_a(
        emu, string, num_chars, flags, ptr, inout_sz,
    );
    stdcall_ret(emu, 7);
}

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
