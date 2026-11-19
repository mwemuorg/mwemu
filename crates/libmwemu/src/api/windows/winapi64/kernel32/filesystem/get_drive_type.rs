use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeA", false);
}

fn get_drive_type(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path = strings::read_text(emu, api_try!(emu, call.arg(emu, 0)), wide);
    log_red!(emu, "kernel32!{api} '{}'", path);
    emu.set_last_error(0);
    call.set_return(emu, 3); // DRIVE_FIXED
}

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeW", true);
}
