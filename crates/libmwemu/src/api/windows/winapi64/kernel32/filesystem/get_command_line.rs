use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineA", false);
}

fn get_command_line(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "kernel32!{api}");

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!{api}: out of memory");
            call.set_return(emu, 0);
            return;
        }
    };

    let exe_name = emu.cfg.exe_name.clone();
    strings::write_string(emu, addr, &exe_name, wide);
    call.set_return(emu, addr);
}

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineW", true);
}
