use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserA", false);
}

fn create_process_as_user(emu: &mut emu::Emu, api: &str, wide: bool) {
    let app_name_ptr = emu.regs().rcx;
    let cmd_line_ptr = emu.regs().rdx;
    let proc_attrs = emu.regs().r8;
    let thread_attrs = emu.regs().r9;
    let inherit = WinApiAbi::arg(emu, 5);
    let flags = WinApiAbi::arg(emu, 6);
    let env = WinApiAbi::arg(emu, 7);
    let cur_dir_ptr = WinApiAbi::arg(emu, 8);
    let startup_info = WinApiAbi::arg(emu, 9);
    let proc_info = WinApiAbi::arg(emu, 10);
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
    emu.regs_mut().rax = 1;
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    create_process_as_user(emu, "CreateProcessAsUserW", true);
}
