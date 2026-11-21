use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn WinHttpConnect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let session = api_try!(emu, call.arg_u32(emu, 0));
    let server_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let port = api_try!(emu, call.arg_u16(emu, 2));
    let server = if server_ptr != 0 {
        emu.maps.read_wide_string(server_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "winhttp!WinHttpConnect session: 0x{:x} server: {} port: {}",
        session,
        server,
        port
    );
    call.set_return(
        emu,
        helper::handler_create(&format!("winhttp://{}:{}", server, port)),
    );
    call.cleanup_args(emu, 4);
}
