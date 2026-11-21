use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SetWindowsHookExA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let id_hook = api_try!(emu, call.arg_i32(emu, 0));
    let lpfn = api_try!(emu, call.arg(emu, 1));
    let hmod = api_try!(emu, call.arg(emu, 2));
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
}

pub fn SetWindowsHookExW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("hook");
    log_red!(
        emu,
        "user32!SetWindowsHookExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
        s,
        p1,
        p2,
        p3,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, hndl);
}
