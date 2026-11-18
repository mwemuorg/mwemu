use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn Module32First(emu: &mut emu::Emu) {
    module32_first(emu, "Module32First");
}

fn module32_first(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let snap = api_try!(emu, call.arg_u32(emu, 0));
    let entry = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "kernel32!{api} snap:0x{:x} entry:0x{:x}", snap, entry);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 0);
}

pub fn Module32FirstW(emu: &mut emu::Emu) {
    module32_first(emu, "Module32FirstW");
}
