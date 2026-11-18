use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn WinHttpOpenRequest(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let connect = api_try!(emu, call.arg_u32(emu, 0));
    let verb_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let object_ptr = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let verb = if verb_ptr != 0 {
        emu.maps.read_wide_string(verb_ptr)
    } else {
        "GET".to_string()
    };
    let object = if object_ptr != 0 {
        emu.maps.read_wide_string(object_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "winhttp!WinHttpOpenRequest connect: 0x{:x} {} {}",
        connect,
        verb,
        object
    );
    call.set_return(
        emu,
        helper::handler_create(&format!("winhttp://request/{} {}", verb, object)),
    );
    call.cleanup_args(emu, 7);
}
