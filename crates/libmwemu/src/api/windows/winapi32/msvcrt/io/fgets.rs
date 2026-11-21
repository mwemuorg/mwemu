use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn fgets(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buf = api_try!(emu, call.arg(emu, 0));
    let n = api_try!(emu, call.arg_u32(emu, 1));
    let stream = api_try!(emu, call.arg(emu, 2));
    let name = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!fgets buf:0x{:x} n:{} stream:0x{:x} '{}'",
        buf,
        n,
        stream,
        name
    );
    if buf != 0 && n > 0 {
        emu.maps.write_byte(buf, b'\n');
        if n > 1 {
            emu.maps.write_byte(buf + 1, 0);
        }
        call.set_return(emu, buf);
    } else {
        call.set_return(emu, 0);
    }
}
