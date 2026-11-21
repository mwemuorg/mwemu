use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ShellExecuteA(emu: &mut emu::Emu) {
    shell_execute(emu, "ShellExecuteA", false);
}

fn shell_execute(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let lp_operation = api_try!(emu, call.arg(emu, 1));
    let lp_file = api_try!(emu, call.arg(emu, 2));
    let lp_parameters = api_try!(emu, call.arg(emu, 3));
    let lp_directory = api_try!(emu, call.arg(emu, 4));
    let n_show_cmd = api_try!(emu, call.arg(emu, 5));

    let operation = if lp_operation != 0 {
        strings::read_text(emu, lp_operation, wide)
    } else {
        "open".to_string()
    };
    let file = strings::read_text(emu, lp_file, wide);
    let params = if lp_parameters != 0 {
        strings::read_text(emu, lp_parameters, wide)
    } else {
        "".to_string()
    };

    log_red!(
        emu,
        "shell32!{api} op: {} file: {} params: {}",
        operation,
        file,
        params
    );

    call.cleanup_args(emu, 6);

    call.set_return(emu, 42); // > 32 means success
}

pub fn ShellExecuteW(emu: &mut emu::Emu) {
    shell_execute(emu, "ShellExecuteW", true);
}
