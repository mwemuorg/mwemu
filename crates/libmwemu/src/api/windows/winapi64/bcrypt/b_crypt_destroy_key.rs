use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn BCryptDestroyKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "bcrypt!BCryptDestroyKey key=0x{:x}", h_key);

    crypto_state::destroy_key(h_key);
    helper::handler_close(h_key);
    call.set_return(emu, STATUS_SUCCESS);
}
