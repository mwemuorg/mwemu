use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserA", false);
}

fn create_process_as_user(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _token = emu.regs().rcx;
    let app_name_ptr = emu.regs().rdx;
    let cmd_line_ptr = emu.regs().r8;
    let _proc_attrs = emu.regs().r9;
    let _thread_attrs = WinApiAbi::arg(emu, 4);
    let _inherit_handles = WinApiAbi::arg(emu, 5);
    let _creation_flags = WinApiAbi::arg(emu, 6);
    let _env = WinApiAbi::arg(emu, 7);
    let _cur_dir = WinApiAbi::arg(emu, 8);
    let _startup_info = WinApiAbi::arg(emu, 9);
    let proc_info = WinApiAbi::arg(emu, 10);
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
    emu.regs_mut().rax = 1;
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserW", true);
}
