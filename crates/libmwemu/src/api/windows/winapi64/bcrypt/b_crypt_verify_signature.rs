use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptVerifySignature(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));
    let _padding_info = api_try!(emu, call.arg(emu, 1));
    let pb_hash = api_try!(emu, call.arg(emu, 2));
    let cb_hash = api_try!(emu, call.arg_u32(emu, 3));
    let pb_signature = api_try!(emu, call.arg(emu, 4));
    let cb_signature = api_try!(emu, call.arg_u32(emu, 5));
    let _flags = api_try!(emu, call.arg_u32(emu, 6));

    log_red!(
        emu,
        "bcrypt!BCryptVerifySignature key:0x{:x} hash:0x{:x}({}) sig:0x{:x}({}) => valid",
        h_key,
        pb_hash,
        cb_hash,
        pb_signature,
        cb_signature
    );
    call.set_return(emu, STATUS_SUCCESS);
}
