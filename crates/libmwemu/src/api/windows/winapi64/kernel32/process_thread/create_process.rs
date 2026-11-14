use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    create_process(emu, "CreateProcessA", false);
}

fn create_process(emu: &mut emu::Emu, api: &str, wide: bool) {
    let appname_ptr = emu.regs().rcx;
    let cmdline_ptr = emu.regs().rdx;
    let _proc_attrs = emu.regs().r8;
    let _thread_attrs = emu.regs().r9;
    let _inherit = WinApiAbi::arg(emu, 4);
    let _creation_flags = WinApiAbi::arg(emu, 5);
    let _env = WinApiAbi::arg(emu, 6);
    let _cwd = WinApiAbi::arg(emu, 7);
    let _startup_info = WinApiAbi::arg(emu, 8);
    let proc_info = WinApiAbi::arg(emu, 9);

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

    emu.regs_mut().rax = 1;
}

pub fn CreateProcessW(emu: &mut emu::Emu) {
    create_process(emu, "CreateProcessW", true);
}
