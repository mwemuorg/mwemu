use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptSIPRemoveSignedDataMsg(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_subject_info = api_try!(emu, call.arg(emu, 0));
    let dw_index = api_try!(emu, call.arg(emu, 1));
    let dw_reserved = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptSIPRemoveSignedDataMsg p_subject_info:0x{:x} dw_index:0x{:x} dw_reserved:0x{:x}",
        p_subject_info,
        dw_index,
        dw_reserved
    );
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
