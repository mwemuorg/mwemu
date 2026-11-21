use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn gethostbyaddr(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let addr_ptr = api_try!(emu, call.arg(emu, 0));
    let len = api_try!(emu, call.arg_u32(emu, 1));
    let typ = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "ws2_32!gethostbyaddr addr:0x{:x} len:{} type:{}",
        addr_ptr,
        len,
        typ
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
