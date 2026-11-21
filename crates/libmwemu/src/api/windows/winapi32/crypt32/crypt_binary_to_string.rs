use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crypt32::{crypt_binary_to_string_a, crypt_binary_to_string_w};
use crate::winapi::policy::api_try;

pub(super) fn CryptBinaryToStringA(emu: &mut emu::Emu) {
    crypt_binary_to_string(emu, "CryptBinaryToStringA", false);
}

pub(super) fn CryptBinaryToStringW(emu: &mut emu::Emu) {
    crypt_binary_to_string(emu, "CryptBinaryToStringW", true);
}

fn crypt_binary_to_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let data_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let data_len = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let flags = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let str_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let str_len_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    if wide {
        let return_value =
            crypt_binary_to_string_w(emu, data_ptr, data_len, flags, str_ptr, str_len_ptr);
        call.set_return(emu, return_value);
    } else {
        let return_value =
            crypt_binary_to_string_a(emu, data_ptr, data_len, flags, str_ptr, str_len_ptr);
        call.set_return(emu, return_value);
    }

    call.cleanup_args(emu, 5);
}
