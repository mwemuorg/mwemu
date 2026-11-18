use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CoIsOle1Class(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let clsid_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!CoIsOle1Class clsid:0x{:x}", clsid_ptr);
    if clsid_ptr != 0 && emu.maps.is_mapped(clsid_ptr) {
        let _ = emu.maps.read_dword(clsid_ptr).unwrap_or(0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
