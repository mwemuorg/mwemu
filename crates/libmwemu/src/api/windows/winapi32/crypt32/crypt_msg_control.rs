use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptMsgControl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_crypt_msg = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    let dw_ctrl_type = api_try!(emu, call.arg(emu, 2));
    let pv_ctrl_para = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!CryptMsgControl 0x{:x}", h_crypt_msg);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
