use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptMsgOpenToEncode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_msg_encoding = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let dw_msg_type = api_try!(emu, call.arg(emu, 2));
    let pv_msg_encode_info = api_try!(emu, call.arg(emu, 3));
    let psz_inner_content_obj_id = api_try!(emu, call.arg(emu, 4));
    log_red!(emu, "crypt32!CryptMsgOpenToEncode 0x{:x}", dw_msg_encoding);
    let hndl = helper::handler_create("CryptMsgOpenToEncode");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 6);
}
