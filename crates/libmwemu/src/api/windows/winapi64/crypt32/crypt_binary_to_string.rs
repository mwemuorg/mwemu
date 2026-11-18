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
    let data_ptr = api_try!(emu, call.arg(emu, 0));
    let data_len = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let str_ptr = api_try!(emu, call.arg(emu, 3));
    let str_len_ptr = api_try!(emu, call.arg(emu, 4));
    if wide {
        let return_value =
            crypt_binary_to_string_w(emu, data_ptr, data_len, flags, str_ptr, str_len_ptr);
        call.set_return(emu, return_value);
    } else {
        let return_value =
            crypt_binary_to_string_a(emu, data_ptr, data_len, flags, str_ptr, str_len_ptr);
        call.set_return(emu, return_value);
    }
}
