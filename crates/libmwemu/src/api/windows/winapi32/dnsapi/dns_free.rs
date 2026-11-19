use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DnsFree(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let data = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    log_red!(emu, "dnsapi!DnsFree ptr:0x{:x}", data);
    call.cleanup_args(emu, 2);
}
