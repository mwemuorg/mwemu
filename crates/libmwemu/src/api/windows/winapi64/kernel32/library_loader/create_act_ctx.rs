use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateActCtxA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_act_ctx = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "kernel32!CreateActCtxA pActCtx:0x{:x}", p_act_ctx);
    call.set_return(emu, helper::handler_create("actctx://"));
}

pub fn CreateActCtxW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    log_red!(emu, "kernel32!CreateActCtxW");
    call.set_return(emu, helper::handler_create("CreateActCtxW"));
}
