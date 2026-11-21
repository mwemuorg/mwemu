use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SafeArrayCreateVector(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let vt = api_try!(emu, call.arg(emu, 0));
    let lower = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let hndl = helper::handler_create(&format!("safearray_vec_{}", count));
    log_red!(
        emu,
        "oleaut32!SafeArrayCreateVector vt:{} lower:{} count:{} => 0x{:x}",
        vt,
        lower,
        count,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
