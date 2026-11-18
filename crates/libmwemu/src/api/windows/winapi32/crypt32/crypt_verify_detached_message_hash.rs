use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyDetachedMessageHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_hash_para = api_try!(emu, call.arg(emu, 0));
    let pb_detached_hash_blob = api_try!(emu, call.arg(emu, 1));
    let cb_detached_hash_blob = api_try!(emu, call.arg(emu, 2));
    let c_to_be_hashed = api_try!(emu, call.arg(emu, 3));
    let rgb_pb_to_be_hashed = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptVerifyDetachedMessageHash 0x{:x}",
        p_hash_para
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
