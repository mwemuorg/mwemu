use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDestroyKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "advapi32!CryptDestroyKey key: 0x{:x}", hkey);
    crypto_state::destroy_key(hkey);
    helper::handler_close(hkey);
    call.cleanup_args(emu, 1);
    call.set_return(emu, 1);
}
