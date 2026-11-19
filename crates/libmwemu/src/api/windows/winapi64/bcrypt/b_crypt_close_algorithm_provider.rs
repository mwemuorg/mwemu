use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptCloseAlgorithmProvider(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_algorithm = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));

    log_red!(
        emu,
        "bcrypt!BCryptCloseAlgorithmProvider handle: 0x{:x} flags: 0x{:x}",
        h_algorithm,
        flags
    );

    helper::handler_close(h_algorithm);
    call.set_return(emu, STATUS_SUCCESS);
}
