use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn EtwEventRegister(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let provider_guid = api_try!(emu, call.arg(emu, 0));
    let callback = api_try!(emu, call.arg(emu, 1));
    let _callback_ctx = api_try!(emu, call.arg(emu, 2));
    let reg_handle_ptr = api_try!(emu, call.arg(emu, 3));
    let guid_str = emu.maps.read_string(provider_guid);
    let hndl = helper::handler_create(&guid_str);
    if reg_handle_ptr != 0 {
        emu.maps.write_qword(reg_handle_ptr, hndl);
    }
    log_red!(
        emu,
        "kernel32!EventRegister guid:'{}' cb:0x{:x} => handle:0x{:x}",
        guid_str,
        callback,
        hndl
    );
    call.set_return(emu, 0);
}
