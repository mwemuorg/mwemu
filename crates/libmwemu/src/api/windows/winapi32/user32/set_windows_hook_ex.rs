use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub(super) fn SetWindowsHookExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let id_hook = api_try!(emu, call.arg_i32(emu, 0));
    let lpfn = api_try!(emu, call.arg_u32(emu, 1));
    let hmod = api_try!(emu, call.arg_u32(emu, 2));
    let thread_id = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "user32!SetWindowsHookExA idHook={} lpfn=0x{:x} hMod=0x{:x} threadId={}",
        id_hook,
        lpfn,
        hmod,
        thread_id
    );
    call.set_return(emu, helper::handler_create("hook://"));
    call.cleanup_args(emu, 4);
}

pub fn SetWindowsHookExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(
        emu,
        "user32!SetWindowsHookExW arg0:0x{:x} arg1:0x{:x}",
        a0,
        a1
    );
    call.set_return(emu, helper::handler_create("setwindowshookexw"));
    call.cleanup_args(emu, 4);
}
