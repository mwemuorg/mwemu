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
    let _token = api_try!(emu, call.arg(emu, 0));
    let app_name_ptr = api_try!(emu, call.arg(emu, 1));
    let cmd_line_ptr = api_try!(emu, call.arg(emu, 2));
    let _proc_attrs = api_try!(emu, call.arg(emu, 3));
    let _thread_attrs = api_try!(emu, call.arg(emu, 4));
    let _inherit_handles = api_try!(emu, call.arg(emu, 5));
    let _creation_flags = api_try!(emu, call.arg(emu, 6));
    let _env = api_try!(emu, call.arg(emu, 7));
    let _cur_dir = api_try!(emu, call.arg(emu, 8));
    let _startup_info = api_try!(emu, call.arg(emu, 9));
    let proc_info = api_try!(emu, call.arg(emu, 10));
    let app = if app_name_ptr != 0 && app_name_ptr > 0xffff {
        strings::read_text(emu, app_name_ptr, wide)
    } else {
        String::new()
    };
    let cmd = if cmd_line_ptr != 0 && cmd_line_ptr > 0xffff {
        strings::read_text(emu, cmd_line_ptr, wide)
    } else {
        String::new()
    };
    if proc_info != 0 {
        let fake_pid = 0x1234u32;
        let fake_tid = 0x1235u32;
        let h_proc = helper::handler_create("process");
        let h_thread = helper::handler_create("thread");
        emu.maps.write_qword(proc_info, h_proc);
        emu.maps.write_qword(proc_info + 8, h_thread);
        emu.maps.write_dword(proc_info + 16, fake_pid);
        emu.maps.write_dword(proc_info + 20, fake_tid);
    }
    log_red!(emu, "advapi32!{api} app:'{}' cmd:'{}'", app, cmd);
    call.set_return(emu, 1);
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserW", true);
}
