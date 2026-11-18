use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn _GetMUIStringFromCache(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let param1 = api_try!(emu, call.arg(emu, 0));
    let param2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernelbase!_GetMUIStringFromCache {}:0x{:x} {}:0x{:x}",
        "param1",
        param1,
        "param2",
        param2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // success
}
