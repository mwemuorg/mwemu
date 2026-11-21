use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _read(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "wincrt!_read fd:{} buf:0x{:x} count:{}",
        fd,
        buf,
        count
    );
    if buf != 0 && count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
