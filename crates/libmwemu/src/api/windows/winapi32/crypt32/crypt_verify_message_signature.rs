use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyMessageSignature(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_verify_para = api_try!(emu, call.arg(emu, 0));
    let dw_signer_index = api_try!(emu, call.arg(emu, 1));
    let pb_signed_blob = api_try!(emu, call.arg(emu, 2));
    let cb_signed_blob = api_try!(emu, call.arg(emu, 3));
    let ppb_decoded = api_try!(emu, call.arg(emu, 4));
    let pcb_decoded = api_try!(emu, call.arg(emu, 5));
    let pp_signer_cert = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "crypt32!CryptVerifyMessageSignature 0x{:x}",
        p_verify_para
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}
