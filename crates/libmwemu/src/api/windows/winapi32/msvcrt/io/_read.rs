use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn _read(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    let name = helper::handler_get_uri(fd);
    log_red!(
        emu,
        "msvcrt!_read fd:{} '{}' buf:0x{:x} count:{}",
        fd,
        name,
        buf,
        count
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
