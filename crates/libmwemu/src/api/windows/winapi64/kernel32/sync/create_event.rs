use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

/*
HANDLE CreateEventA(
  [in, optional] LPSECURITY_ATTRIBUTES lpEventAttributes,
  [in]           BOOL                  bManualReset,
  [in]           BOOL                  bInitialState,
  [in, optional] LPCSTR                lpName
);

*/
pub fn CreateEventA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let attributes = api_try!(emu, call.arg(emu, 0));
    let bManualReset = api_try!(emu, call.arg(emu, 1));
    let bInitialState = api_try!(emu, call.arg(emu, 2));
    let name_ptr = api_try!(emu, call.arg(emu, 3));

    let mut name = String::new();
    if name_ptr > 0 {
        name = emu.maps.read_string(name_ptr);
    }

    log_red!(
        emu,
        "kernel32!CreateEventA attr: 0x{:x} manual_reset: {} init_state: {} name: {}",
        attributes,
        bManualReset,
        bInitialState,
        name
    );

    let uri = format!("event://{}", name);
    call.set_return(emu, helper::handler_create(&uri));
}

pub fn CreateEventW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let attributes = api_try!(emu, call.arg(emu, 0));
    let manual_reset = api_try!(emu, call.arg(emu, 1));
    let initial_state = api_try!(emu, call.arg(emu, 2));
    let name_ptr = api_try!(emu, call.arg(emu, 3));

    let mut name = String::new();
    if name_ptr > 0 {
        name = emu.maps.read_wide_string(name_ptr);
    }

    log_red!(
        emu,
        "kernel32!CreateEventW attr: 0x{:x} manual_reset: {} init_state: {} name: {}",
        attributes,
        manual_reset,
        initial_state,
        name
    );

    let uri = format!("event://{}", name);
    call.set_return(emu, helper::handler_create(&uri));
}
