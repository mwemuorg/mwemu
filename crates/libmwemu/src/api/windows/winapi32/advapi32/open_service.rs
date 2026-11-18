use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenServiceA(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceA", false);
}

fn open_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hsc = api_try!(emu, call.arg_u32(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let access = api_try!(emu, call.arg_u32(emu, 2));
    let name = strings::read_text(emu, name_ptr, wide);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!{api} scm:0x{:x} '{}' access:0x{:x} => 0x{:x}",
        hsc,
        name,
        access,
        hsvc
    );
    call.cleanup_args(emu, 3);
    call.set_return(emu, hsvc);
}

pub fn OpenServiceW(emu: &mut emu::Emu) {
    open_service(emu, "OpenServiceW", true);
}
