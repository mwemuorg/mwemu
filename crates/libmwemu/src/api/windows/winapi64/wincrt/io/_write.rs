use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _write(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg_u32(emu, 2));
    let data = emu.maps.read_string(buf);
    log_red!(
        emu,
        "wincrt!_write fd:{} count:{} data:'{}'",
        fd,
        count,
        data
    );
    call.set_return(emu, count as u64);
}
