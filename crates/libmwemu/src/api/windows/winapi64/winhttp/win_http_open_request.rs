use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WinHttpOpenRequest(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_connect = api_try!(emu, call.arg(emu, 0));
    let verb_ptr = api_try!(emu, call.arg(emu, 1));
    let object_ptr = api_try!(emu, call.arg(emu, 2));
    let _version_ptr = api_try!(emu, call.arg(emu, 3));

    let verb = if verb_ptr == 0 {
        "GET".to_string()
    } else {
        emu.maps.read_wide_string(verb_ptr)
    };

    let object = if object_ptr == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string(object_ptr)
    };

    log_red!(
        emu,
        "winhttp!WinHttpOpenRequest connect: 0x{:x} {} {}",
        h_connect,
        verb,
        object
    );

    if !helper::handler_exist(h_connect) {
        log::trace!("\tinvalid connect handle");
    }

    let uri = format!("winhttp://request/{} {}", verb, object);
    call.set_return(emu, helper::handler_create(&uri));
}
