use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptDecodeObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 0));
    let lpsz_struct_type = api_try!(emu, call.arg(emu, 1));
    let pb_encoded = api_try!(emu, call.arg(emu, 2));
    let cb_encoded = api_try!(emu, call.arg(emu, 3));
    let dw_flags = api_try!(emu, call.arg(emu, 4));
    let pv_struct_info = api_try!(emu, call.arg(emu, 5));
    let pcb_struct_info = api_try!(emu, call.arg(emu, 6));
    log_red!(emu, "crypt32!CryptDecodeObject 0x{:x}", dw_cert_encoding);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}
