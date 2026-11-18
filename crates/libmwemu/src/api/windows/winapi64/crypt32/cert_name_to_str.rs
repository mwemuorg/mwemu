use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertNameToStr(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let buf_size = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!CertNameToStr arg1:0x{:x}", arg1);
    if buf_ptr != 0 {
        emu.maps.write_string(buf_ptr, "CN=localhost");
    }
    emu.set_last_error(0);
    call.set_return(emu, 13);
}
