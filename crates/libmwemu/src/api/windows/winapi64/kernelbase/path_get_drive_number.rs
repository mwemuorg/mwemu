use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn PathGetDriveNumberA(emu: &mut emu::Emu) {
    path_get_drive_number(emu, "PathGetDriveNumberA", false);
}

fn path_get_drive_number(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };
    let result = if path.len() >= 2 && path.as_bytes()[1] == b':' {
        let ch = path.as_bytes()[0].to_ascii_uppercase();
        if ch.is_ascii_uppercase() {
            (ch - b'A') as u64
        } else {
            u64::MAX
        }
    } else {
        u64::MAX
    };
    log_red!(emu, "kernelbase!{api} '{}' => {}", path, result as i64);
    emu.set_last_error(0);
    call.set_return(emu, result);
}

pub fn PathGetDriveNumberW(emu: &mut emu::Emu) {
    path_get_drive_number(emu, "PathGetDriveNumberW", true);
}
