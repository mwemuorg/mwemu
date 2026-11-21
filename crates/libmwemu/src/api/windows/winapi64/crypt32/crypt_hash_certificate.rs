use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crypt32::crypt_hash_certificate;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptHashCertificate(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let alg_id = api_try!(emu, call.arg(emu, 1));
    let cert_ptr = api_try!(emu, call.arg(emu, 2));
    let cert_len = api_try!(emu, call.arg(emu, 3));
    let hash_ptr = api_try!(emu, call.arg(emu, 4));
    let hash_len_ptr = api_try!(emu, call.arg(emu, 5));
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    let return_value = crypt_hash_certificate(
        emu,
        alg_id as u32,
        cert_ptr,
        cert_len,
        hash_ptr,
        hash_len_ptr,
    );
    call.set_return(emu, return_value);
}
