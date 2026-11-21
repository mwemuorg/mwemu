use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DnsFree(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let data = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "dnsapi!DnsFree ptr:0x{:x}", data);
}
