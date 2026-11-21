use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;

fn convert(emu: &mut Emu, api: &str, short: bool) {
    let call = WinApi(Stdcall);
    // Windows x86/x64 are little-endian regardless of the emulator host.
    let result = if short {
        api_try!(emu, call.arg_u16(emu, 0)).swap_bytes() as u64
    } else {
        api_try!(emu, call.arg_u32(emu, 0)).swap_bytes() as u64
    };
    log_red!(emu, "ws2_32!{api} => {result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn htons(emu: &mut Emu) {
    convert(emu, "htons", true);
}

pub fn ntohs(emu: &mut Emu) {
    convert(emu, "ntohs", true);
}

pub fn htonl(emu: &mut Emu) {
    convert(emu, "htonl", false);
}

pub fn ntohl(emu: &mut Emu) {
    convert(emu, "ntohl", false);
}
