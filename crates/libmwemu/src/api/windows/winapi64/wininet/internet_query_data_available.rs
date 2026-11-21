use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetQueryDataAvailable(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let bytes_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "wininet!InternetQueryDataAvailable handle:0x{:x}",
        handle
    );
    if bytes_ptr != 0 {
        emu.maps.write_dword(bytes_ptr, 0);
    }
    call.set_return(emu, 1);
}
