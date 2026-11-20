use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let callback = api_try!(emu, call.arg(emu, 0));

    log_red!(
        emu,
        "kernel32!SetUnhandledExceptionFilter  callback=0x{:x}",
        callback
    );

    let return_value = emu.uef();
    call.set_return(emu, return_value);
    emu.set_uef(callback);

    call.cleanup_args(emu, 1);
}
