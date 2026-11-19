use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetDriveTypeA(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeA", false);
}

fn get_drive_type(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let root_ptr = api_try!(emu, call.arg(emu, 0));
    let root = if root_ptr != 0 {
        strings::read_text(emu, root_ptr, wide)
    } else {
        String::from("C:\\")
    };
    log_red!(emu, "kernelbase!{api} '{}' => DRIVE_FIXED", root);
    emu.set_last_error(0);
    call.set_return(emu, 3);
}

pub fn GetDriveTypeW(emu: &mut emu::Emu) {
    get_drive_type(emu, "GetDriveTypeW", true);
}
