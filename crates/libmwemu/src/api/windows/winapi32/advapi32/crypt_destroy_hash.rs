use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDestroyHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "advapi32!CryptDestroyHash hash: 0x{:x}", hhash);
    crypto_state::destroy_hash(hhash);
    helper::handler_close(hhash);
    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}
