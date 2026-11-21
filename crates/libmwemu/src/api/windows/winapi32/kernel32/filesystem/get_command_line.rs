use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineA", false);
}

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineW", true);
}

fn get_command_line(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    if wide {
        log_red!(emu, "kernel32!GetCommandlineW");
    } else {
        log_red!(emu, "kernel32!GetCommandlineA");
    }

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!{api}: out of memory");
            call.set_return(emu, 0);
            return;
        }
    };
    strings::write_string(emu, addr, "test.exe", wide);
    call.set_return(emu, addr);
}
