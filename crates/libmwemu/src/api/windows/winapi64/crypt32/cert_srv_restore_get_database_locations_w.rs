use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertSrvRestoreGetDatabaseLocationsW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = emu.maps.read_wide_string(str_ptr);
    log_red!(emu, "crypt32!CertSrvRestoreGetDatabaseLocationsW '{}'", s);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
