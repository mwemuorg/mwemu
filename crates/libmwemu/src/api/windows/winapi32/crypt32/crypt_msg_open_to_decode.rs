use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptMsgOpenToDecode(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_msg_encoding = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let dw_msg_type = api_try!(emu, call.arg(emu, 2));
    let h_crypt_prov = api_try!(emu, call.arg(emu, 3));
    let p_recipient_info = api_try!(emu, call.arg(emu, 4));
    log_red!(emu, "crypt32!CryptMsgOpenToDecode 0x{:x}", dw_msg_encoding);
    let hndl = helper::handler_create("CryptMsgOpenToDecode");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 6);
}
