use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WinHttpConnect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_session = api_try!(emu, call.arg(emu, 0));
    let server_ptr = api_try!(emu, call.arg(emu, 1));
    let port = api_try!(emu, call.arg_u16(emu, 2));

    let server = if server_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(server_ptr)
    };

    log_red!(
        emu,
        "winhttp!WinHttpConnect session: 0x{:x} server: {} port: {}",
        h_session,
        server,
        port
    );

    if !helper::handler_exist(h_session) {
        log::trace!("\tinvalid session handle");
    }

    let uri = format!("winhttp://{}:{}", server, port);
    call.set_return(emu, helper::handler_create(&uri));
}
