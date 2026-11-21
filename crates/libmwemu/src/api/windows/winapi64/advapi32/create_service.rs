use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateServiceA(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceA", false);
}

fn create_service(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hsc = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let display_ptr = api_try!(emu, call.arg(emu, 2));
    let access = api_try!(emu, call.arg(emu, 3));
    let name = strings::read_text(emu, name_ptr, wide);
    let display = strings::read_text(emu, display_ptr, wide);
    let hsvc = helper::handler_create(&name);
    log_red!(
        emu,
        "advapi32!{api} '{}' display:'{}' => 0x{:x}",
        name,
        display,
        hsvc
    );
    emu.set_last_error(0);
    call.set_return(emu, hsvc);
}

pub fn CreateServiceW(emu: &mut emu::Emu) {
    create_service(emu, "CreateServiceW", true);
}
