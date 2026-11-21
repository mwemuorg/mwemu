use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptGetObjectUrl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psz_url_oid = api_try!(emu, call.arg(emu, 0));
    let pv_para = api_try!(emu, call.arg(emu, 1));
    let dw_flags = api_try!(emu, call.arg(emu, 2));
    let p_url_array = api_try!(emu, call.arg(emu, 3));
    let pcb_url_array = api_try!(emu, call.arg(emu, 4));
    let p_url_info = api_try!(emu, call.arg(emu, 5));
    let pcb_url_info = api_try!(emu, call.arg(emu, 6));
    let pv_reserved = api_try!(emu, call.arg(emu, 7));
    log_red!(emu, "crypt32!CryptGetObjectUrl 0x{:x}", psz_url_oid);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
