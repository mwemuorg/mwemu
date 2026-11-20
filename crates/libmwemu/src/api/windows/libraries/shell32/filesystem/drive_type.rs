use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::files::drive_number_type;
use crate::winapi::policy::api_try;

fn query(emu: &mut Emu, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let drive = api_try!(emu, call.arg_i32(emu, 0));
    if extended {
        let _reserved = api_try!(emu, call.arg_u32(emu, 1));
    }
    let value = drive_number_type(drive);
    log_red!(emu, "shell32!{api} drive={drive} type={value}");
    call.set_return(emu, value);
    call.cleanup_args(emu, if extended { 2 } else { 1 });
}

pub fn DriveType(emu: &mut Emu) {
    query(emu, "DriveType", false);
}
pub fn RealDriveType(emu: &mut Emu) {
    query(emu, "RealDriveType", true);
}
