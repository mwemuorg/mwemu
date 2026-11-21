use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSIPVerifyIndirectData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_subject_info = api_try!(emu, call.arg(emu, 0));
    let p_indirect_data = api_try!(emu, call.arg(emu, 1));
    let dw_reserved = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptSIPVerifyIndirectData p_subject_info:0x{:x} p_indirect_data:0x{:x} dw_reserved:0x{:x}",
        p_subject_info,
        p_indirect_data,
        dw_reserved
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
