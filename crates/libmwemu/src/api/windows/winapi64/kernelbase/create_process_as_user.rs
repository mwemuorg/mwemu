use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserA", false);
}

fn create_process_as_user(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let app_name_ptr = api_try!(emu, call.arg(emu, 0));
    let cmd_line_ptr = api_try!(emu, call.arg(emu, 1));
    let proc_attrs = api_try!(emu, call.arg(emu, 2));
    let thread_attrs = api_try!(emu, call.arg(emu, 3));
    let inherit = api_try!(emu, call.arg(emu, 5));
    let flags = api_try!(emu, call.arg(emu, 6));
    let env = api_try!(emu, call.arg(emu, 7));
    let cur_dir_ptr = api_try!(emu, call.arg(emu, 8));
    let startup_info = api_try!(emu, call.arg(emu, 9));
    let proc_info = api_try!(emu, call.arg(emu, 10));
    let app = if app_name_ptr != 0 {
        strings::read_text(emu, app_name_ptr, wide)
    } else {
        String::new()
    };
    let cmd = if cmd_line_ptr != 0 {
        strings::read_text(emu, cmd_line_ptr, wide)
    } else {
        String::new()
    };
    if proc_info != 0 {
        let hndl = helper::handler_create(&format!("process_{}", app));
        let thread_hndl = helper::handler_create("process_thread");
        emu.maps.write_dword(proc_info + 8, 0x1000); // dwProcessId
        emu.maps.write_dword(proc_info + 12, 0x2000); // dwThreadId
    }
    log_red!(emu, "kernelbase!{api} app:'{}' cmd:'{}'", app, cmd);
    call.set_return(emu, 1);
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserW", true);
}
