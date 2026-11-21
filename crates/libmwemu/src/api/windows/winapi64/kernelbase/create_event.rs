use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateEventA(emu: &mut emu::Emu) {
    create_event(emu, "CreateEventA", false);
}

fn create_event(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let security = api_try!(emu, call.arg(emu, 0));
    let manual_reset = api_try!(emu, call.arg(emu, 1));
    let initial_state = api_try!(emu, call.arg(emu, 2));
    let name_ptr = api_try!(emu, call.arg(emu, 3));
    let name = if name_ptr != 0 {
        strings::read_text(emu, name_ptr, wide)
    } else {
        String::from("unnamed")
    };
    let hndl = helper::handler_create(&format!("event_{}", name));
    log_red!(
        emu,
        "kernelbase!{api} '{}' manual:{} initial:{} => 0x{:x}",
        name,
        manual_reset,
        initial_state,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}

pub fn CreateEventW(emu: &mut emu::Emu) {
    create_event(emu, "CreateEventW", true);
}
