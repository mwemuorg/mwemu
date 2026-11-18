use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    create_process(emu, "CreateProcessA", false);
}

fn create_process(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let appname_ptr = api_try!(emu, call.arg(emu, 0));
    let cmdline_ptr = api_try!(emu, call.arg(emu, 1));
    let _proc_attrs = api_try!(emu, call.arg(emu, 2));
    let _thread_attrs = api_try!(emu, call.arg(emu, 3));
    let _inherit = api_try!(emu, call.arg(emu, 4));
    let _creation_flags = api_try!(emu, call.arg(emu, 5));
    let _env = api_try!(emu, call.arg(emu, 6));
    let _cwd = api_try!(emu, call.arg(emu, 7));
    let _startup_info = api_try!(emu, call.arg(emu, 8));
    let proc_info = api_try!(emu, call.arg(emu, 9));

    let appname = strings::read_text(emu, appname_ptr, wide);
    let cmdline = strings::read_text(emu, cmdline_ptr, wide);

    log_red!(emu, "kernel32!{api} '{}' '{}'", appname, cmdline);

    if proc_info != 0 {
        let h_process = helper::handler_create(&format!("process://{}", appname));
        let h_thread = helper::handler_create(&format!("thread://main_{}", appname));
        emu.maps.write_qword(proc_info, h_process);
        emu.maps.write_qword(proc_info + 8, h_thread);
        emu.maps.write_dword(proc_info + 16, 1234);
        emu.maps.write_dword(proc_info + 20, 5678);
    }

    call.set_return(emu, 1);
}

pub fn CreateProcessW(emu: &mut emu::Emu) {
    create_process(emu, "CreateProcessW", true);
}
