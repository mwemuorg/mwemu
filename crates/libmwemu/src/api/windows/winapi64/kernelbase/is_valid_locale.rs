use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernelbase!IsValidLocale locale:0x{:x} flags:0x{:x}",
        locale,
        flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
