use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn OpenMutexA(emu: &mut emu::Emu) {
    open_mutex(emu, "OpenMutexA", false);
}

pub fn OpenMutexW(emu: &mut emu::Emu) {
    open_mutex(emu, "OpenMutexW", true);
}

fn open_mutex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let access = api_try!(emu, call.arg(emu, 0));
    let inherit = api_try!(emu, call.arg(emu, 1));
    let name = api_try!(emu, call.arg(emu, 2));
    let name_s = if name != 0 {
        strings::read_text(emu, name, wide)
    } else {
        String::new()
    };
    if wide {
        let hndl = helper::handler_create(&format!("mutex://{}", name_s));
        log_red!(
            emu,
            "kernel32!{api} access:0x{:x} name:'{}' => 0x{:x}",
            access,
            name_s,
            hndl
        );
        call.set_return(emu, hndl);
        emu.set_last_error(0);
    } else {
        log_red!(
            emu,
            "kernel32!{api} access:0x{:x} name:'{}' => 0",
            access,
            name_s
        );
        call.set_return(emu, 0);
        emu.set_last_error(2);
    }
}
