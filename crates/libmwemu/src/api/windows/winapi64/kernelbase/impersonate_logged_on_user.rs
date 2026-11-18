use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ImpersonateLoggedOnUser(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "kernelbase!ImpersonateLoggedOnUser token:0x{:x}",
        token
    );
    call.set_return(emu, 1);
    emu.set_last_error(0);
}
