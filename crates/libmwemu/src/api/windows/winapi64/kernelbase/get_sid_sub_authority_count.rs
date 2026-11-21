use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetSidSubAuthorityCount(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernelbase!GetSidSubAuthorityCount sid:0x{:x}", p0);
    if p0 != 0 && emu.maps.is_mapped(p0) {
        let _revision = emu.maps.read_byte(p0).unwrap_or(0);
        let count = emu.maps.read_byte(p0 + 1).unwrap_or(1);
        call.set_return(emu, p0 + 1);
    } else {
        call.set_return(emu, 0);
    }
    emu.set_last_error(0);
}
