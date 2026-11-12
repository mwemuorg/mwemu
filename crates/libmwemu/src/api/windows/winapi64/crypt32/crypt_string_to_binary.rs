use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptStringToBinary(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    crypt_string_to_binary(emu, "CryptStringToBinaryA", false);
}

pub(super) fn CryptStringToBinaryW(emu: &mut emu::Emu) {
    crypt_string_to_binary(emu, "CryptStringToBinaryW", true);
}

fn crypt_string_to_binary(emu: &mut emu::Emu, api: &str, wide: bool) {
    let string = emu.regs().rcx;
    let num_chars = emu.regs().rdx;
    let flags = emu.regs().r8;
    let ptr = emu.regs().r9;
    let inout_sz = WinApiAbi::arg(emu, 4);
    if wide {
        emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_w(
            emu, string, num_chars, flags, ptr, inout_sz,
        );
    } else {
        emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_string_to_binary_a(
            emu, string, num_chars, flags, ptr, inout_sz,
        );
    }
}
