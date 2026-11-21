use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ProcessIdleTasks(emu: &mut emu::Emu) {
    process_idle_tasks(emu, "ProcessIdleTasks", false);
}

pub fn ProcessIdleTasksW(emu: &mut emu::Emu) {
    process_idle_tasks(emu, "ProcessIdleTasksW", true);
}

fn process_idle_tasks(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    if wide {
        let p0 = api_try!(emu, call.arg(emu, 0));
        let p1 = api_try!(emu, call.arg(emu, 1));
        let s = if p0 != 0 && p0 > 0xffff {
            strings::read_text(emu, p0, wide)
        } else {
            format!("0x{:x}", p0)
        };
        log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    } else {
        let handle = api_try!(emu, call.arg(emu, 0));
        let arg2 = api_try!(emu, call.arg(emu, 1));
        log_red!(emu, "advapi32!{api} h:0x{:x} arg2:0x{:x}", handle, arg2);
        let first_param = api_try!(emu, call.arg(emu, 0));
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
        for _ in 0..2 {}
    }

    emu.set_last_error(0);
    call.set_return(emu, 1);
}
