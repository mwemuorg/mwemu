use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptFindOIDInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_key_type = api_try!(emu, call.arg(emu, 0));
    let pv_key = api_try!(emu, call.arg(emu, 1));
    let dw_group_id = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptFindOIDInfo dw_key_type:0x{:x} pv_key:0x{:x} dw_group_id:0x{:x}",
        dw_key_type,
        pv_key,
        dw_group_id
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
