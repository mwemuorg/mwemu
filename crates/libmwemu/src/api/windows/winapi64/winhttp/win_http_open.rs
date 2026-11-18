use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WinHttpOpen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let agent_ptr = api_try!(emu, call.arg(emu, 0));
    let _access_type = api_try!(emu, call.arg_u32(emu, 1));
    let _proxy_ptr = api_try!(emu, call.arg(emu, 2));
    let _proxy_bypass_ptr = api_try!(emu, call.arg(emu, 3));

    let agent = if agent_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(agent_ptr)
    };

    log_red!(emu, "winhttp!WinHttpOpen agent: {}", agent);

    let uri = format!("winhttp://session/{}", agent);
    call.set_return(emu, helper::handler_create(&uri));
}
