use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDestroyHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg(emu, 0));
    call.cleanup_args(emu, 1);
    log_red!(emu, "crypt32!CryptDestroyHash hash: 0x{:x}", h_hash);
    if h_hash != 0 {
        crypto_state::destroy_hash(h_hash);
        helper::handler_close(h_hash);
    }
    call.set_return(emu, 1); // TRUE
}
