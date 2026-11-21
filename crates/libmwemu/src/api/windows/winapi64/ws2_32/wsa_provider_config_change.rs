use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAProviderConfigChange(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle_ptr = api_try!(emu, call.arg(emu, 0));
    let overlapped_ptr = api_try!(emu, call.arg(emu, 1));
    let completion_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!WSAProviderConfigChange");
    if handle_ptr != 0 {
        let handle = helper::handler_create("wsa_provider_change");
        emu.maps.write_qword(handle_ptr, handle);
    }
    call.set_return(emu, 0);
}
