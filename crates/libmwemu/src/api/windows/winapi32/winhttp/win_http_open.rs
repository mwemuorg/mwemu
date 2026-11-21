use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn WinHttpOpen(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let agent_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let agent = if agent_ptr != 0 {
        emu.maps.read_wide_string(agent_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "winhttp!WinHttpOpen agent: {}", agent);
    call.set_return(
        emu,
        helper::handler_create(&format!("winhttp://session/{}", agent)),
    );
    call.cleanup_args(emu, 5);
}
