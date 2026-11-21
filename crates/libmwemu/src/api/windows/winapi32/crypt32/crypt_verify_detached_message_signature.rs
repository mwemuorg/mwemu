use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyDetachedMessageSignature(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_verify_para = api_try!(emu, call.arg(emu, 0));
    let dw_signer_index = api_try!(emu, call.arg(emu, 1));
    let pb_detached_sign_blob = api_try!(emu, call.arg(emu, 2));
    let cb_detached_sign_blob = api_try!(emu, call.arg(emu, 3));
    let c_to_be_signed = api_try!(emu, call.arg(emu, 4));
    let rgb_pb_to_be_signed = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "crypt32!CryptVerifyDetachedMessageSignature 0x{:x}",
        p_verify_para
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
