use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetTimeZoneInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!SetTimeZoneInformation ptr:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        let _bias = emu.maps.read_dword(p0).unwrap_or(0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
