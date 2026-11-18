use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn EtwEventWriteTransfer(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let reg_handle = api_try!(emu, call.arg(emu, 0));
    let event_descriptor = api_try!(emu, call.arg(emu, 1));
    let activity_id = api_try!(emu, call.arg(emu, 2));
    let _related_id = api_try!(emu, call.arg(emu, 3));
    let desc_str = if event_descriptor != 0 {
        emu.maps.read_string(event_descriptor)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!EventWriteTransfer handle:0x{:x} desc:'{}' activity:0x{:x}",
        reg_handle,
        desc_str,
        activity_id
    );
    call.set_return(emu, 0);
}
