use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenEventLogA(emu: &mut emu::Emu) {
    open_event_log(emu, "OpenEventLogA", false);
}

pub fn OpenEventLogW(emu: &mut emu::Emu) {
    open_event_log(emu, "OpenEventLogW", true);
}

fn open_event_log(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let source_ptr = api_try!(emu, call.arg(emu, 1));
    let source = if source_ptr != 0 && source_ptr > 0xffff {
        strings::read_text(emu, source_ptr, wide)
    } else {
        String::from("Application")
    };
    let hndl = helper::handler_create(&format!("eventlog://{}", source));
    log_red!(emu, "advapi32!{api} source:'{}' => 0x{:x}", source, hndl);
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
