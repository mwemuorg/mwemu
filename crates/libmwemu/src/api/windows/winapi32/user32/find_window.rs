use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn FindWindowA(emu: &mut emu::Emu) {
    find_window(emu, "FindWindowA", false);
}

fn find_window(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let class_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let window_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let class_name = if class_ptr != 0 {
        strings::read_text(emu, class_ptr, wide)
    } else {
        String::new()
    };
    let window_name = if window_ptr != 0 {
        strings::read_text(emu, window_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!{api} class='{}' window='{}'",
        class_name,
        window_name
    );
    call.set_return(emu, 0); // not found
    call.cleanup_args(emu, 2);
}

pub(super) fn FindWindowW(emu: &mut emu::Emu) {
    find_window(emu, "FindWindowW", true);
}
