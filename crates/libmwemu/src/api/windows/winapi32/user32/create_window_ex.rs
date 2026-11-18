use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub(super) fn CreateWindowExA(emu: &mut emu::Emu) {
    create_window_ex(emu, "CreateWindowExA", false);
}

fn create_window_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let class = strings::read_text(emu, api_try!(emu, call.arg_u32(emu, 1)) as u64, wide);
    let title = strings::read_text(emu, api_try!(emu, call.arg_u32(emu, 2)) as u64, wide);
    log_red!(emu, "user32!{api} class:'{}' title:'{}'", class, title);
    call.set_return(emu, 0x1337_0001);
    call.cleanup_args(emu, 12);
}

pub(super) fn CreateWindowExW(emu: &mut emu::Emu) {
    create_window_ex(emu, "CreateWindowExW", true);
}
