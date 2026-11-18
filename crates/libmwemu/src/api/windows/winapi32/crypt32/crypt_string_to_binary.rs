use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crypt32::{crypt_string_to_binary_a, crypt_string_to_binary_w};
use crate::winapi::policy::api_try;

pub fn CryptStringToBinary(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CryptStringToBinary arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

pub fn CryptStringToBinaryA(emu: &mut emu::Emu) {
    crypt_string_to_binary(emu, "CryptStringToBinaryA", false);
}

pub(super) fn CryptStringToBinaryW(emu: &mut emu::Emu) {
    crypt_string_to_binary(emu, "CryptStringToBinaryW", true);
}

fn crypt_string_to_binary(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let string = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let num_chars = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let flags = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let inout_sz = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    if wide {
        let return_value = crypt_string_to_binary_w(emu, string, num_chars, flags, ptr, inout_sz);
        call.set_return(emu, return_value);
    } else {
        let return_value = crypt_string_to_binary_a(emu, string, num_chars, flags, ptr, inout_sz);
        call.set_return(emu, return_value);
    }

    call.cleanup_args(emu, 7);
}
