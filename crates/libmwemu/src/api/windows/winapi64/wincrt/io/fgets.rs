use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn fgets(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let buf = api_try!(emu, call.arg(emu, 0));
    let max_count = api_try!(emu, call.arg_i32(emu, 1));
    let stream = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "wincrt!fgets buf:0x{:x} max:{} stream:0x{:x}",
        buf,
        max_count,
        stream
    );
    if buf != 0 && max_count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
