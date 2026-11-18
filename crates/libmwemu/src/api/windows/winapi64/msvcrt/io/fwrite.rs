use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn fwrite(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buff_ptr = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg_u32(emu, 1));
    let nemb = api_try!(emu, call.arg_u32(emu, 2));
    let file = api_try!(emu, call.arg(emu, 3));
    let filename = helper::handler_get_uri(file);
    log_red!(
        emu,
        "msvcrt!fwrite `{}` 0x{:x} {}",
        filename,
        buff_ptr,
        size
    );
    emu.set_last_error(0);
    call.set_return(emu, size as u64);
}
