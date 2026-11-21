use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptFinalizeKeyPair(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "bcrypt!BCryptFinalizeKeyPair key=0x{:x}", h_key);

    call.set_return(emu, STATUS_SUCCESS);
}
