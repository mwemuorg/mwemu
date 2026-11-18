use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSIPGetSignedDataMsg(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_subject_info = api_try!(emu, call.arg(emu, 0));
    let pdw_encoding_type = api_try!(emu, call.arg(emu, 1));
    let dw_index = api_try!(emu, call.arg(emu, 2));
    let pcb_signed_data_msg = api_try!(emu, call.arg(emu, 3));
    let pb_signed_data_msg = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptSIPGetSignedDataMsg 0x{:x}",
        p_subject_info
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
