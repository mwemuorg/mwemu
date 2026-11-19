use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptHashCertificate2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pwsz_hash_alg = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let pv_reserved = api_try!(emu, call.arg(emu, 2));
    let pb_encoded = api_try!(emu, call.arg(emu, 3));
    let cb_encoded = api_try!(emu, call.arg(emu, 4));
    let pb_computed_hash = api_try!(emu, call.arg(emu, 5));
    log_red!(emu, "crypt32!CryptHashCertificate2 0x{:x}", pwsz_hash_alg);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}
