use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ShellExecuteEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "shell32!ShellExecuteEx arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    call.cleanup_args(emu, 2);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn ShellExecuteExA(emu: &mut emu::Emu) {
    shell_execute_ex(emu, "ShellExecuteExA", false);
}

pub fn ShellExecuteExW(emu: &mut emu::Emu) {
    shell_execute_ex(emu, "ShellExecuteExW", true);
}

fn shell_execute_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let str_ptr = api_try!(emu, call.arg(emu, 0));
    let s = if str_ptr != 0 {
        strings::read_text(emu, str_ptr, wide)
    } else {
        String::new()
    };
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(emu, "shell32!{api} '{}' arg2:0x{:x}", s, arg2);
    if wide {
        let handle = helper::handler_create("shellexecuteexw");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 1);
    } else {
        let handle = helper::handler_create("shellexecuteexa");
        call.set_return(emu, handle);
        call.cleanup_args(emu, 1);
    }
}
