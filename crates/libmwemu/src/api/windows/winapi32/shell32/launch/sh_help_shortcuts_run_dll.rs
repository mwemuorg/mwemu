use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SHHelpShortcuts_RunDLL(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "shell32!SHHelpShortcuts_RunDLL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    call.cleanup_args(emu, 2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn SHHelpShortcuts_RunDLLA(emu: &mut emu::Emu) {
    sh_help_shortcuts_run_dll(emu, "SHHelpShortcuts_RunDLLA", false);
}

fn sh_help_shortcuts_run_dll(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "shell32!{api} '{}'  p1:0x{:x}", s, p1);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}

pub fn SHHelpShortcuts_RunDLLW(emu: &mut emu::Emu) {
    sh_help_shortcuts_run_dll(emu, "SHHelpShortcuts_RunDLLW", true);
}
