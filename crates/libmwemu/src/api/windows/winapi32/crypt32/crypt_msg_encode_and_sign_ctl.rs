use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptMsgEncodeAndSignCTL(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_msg_encoding = api_try!(emu, call.arg(emu, 0));
    let p_ctl_info = api_try!(emu, call.arg(emu, 1));
    let p_sign_info = api_try!(emu, call.arg(emu, 2));
    let dw_flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptMsgEncodeAndSignCTL 0x{:x}",
        dw_msg_encoding
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
