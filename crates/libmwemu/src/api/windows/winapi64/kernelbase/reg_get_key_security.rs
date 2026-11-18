use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn RegGetKeySecurity(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let sec_info = api_try!(emu, call.arg_u32(emu, 1));
    let sec_desc = api_try!(emu, call.arg(emu, 2));
    let cb_sec_desc = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "kernelbase!RegGetKeySecurity hkey:0x{:x} info:0x{:x}",
        hkey,
        sec_info
    );
    if cb_sec_desc != 0 {
        emu.maps.write_dword(cb_sec_desc, 20);
    }
    call.set_return(emu, 122); // ERROR_INSUFFICIENT_BUFFER
}
