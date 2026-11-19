use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptDestroyHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "bcrypt!BCryptDestroyHash hash=0x{:x}", h_hash);
    crypto_state::destroy_hash(h_hash);
    helper::handler_close(h_hash);
    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 1);
}
