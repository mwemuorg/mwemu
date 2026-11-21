use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn tolower(emu: &mut emu::Emu) {
    let call = WinApi(Cdecl);
    let c = api_try!(emu, call.arg_u32(emu, 0));
    let result = if (0x41..=0x5a).contains(&c) {
        c + 0x20
    } else {
        c
    };

    call.set_return(emu, result as u64);
}
