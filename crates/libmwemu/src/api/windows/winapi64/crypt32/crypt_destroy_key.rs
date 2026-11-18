use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptDestroyKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CryptDestroyKey key: 0x{:x}", h_key);
    if h_key != 0 {
        helper::handler_close(h_key);
    }
    call.set_return(emu, 1); // TRUE
}
