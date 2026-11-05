use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessW(emu: &mut emu::Emu) {
    let appname_ptr = emu.regs().rcx;
    let cmdline_ptr = emu.regs().rdx;
    let _proc_attrs = emu.regs().r8;
    let _thread_attrs = emu.regs().r9;
    let _inherit = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _creation_flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _env = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _cwd = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _startup_info = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let proc_info = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);

    let appname = emu.maps.read_wide_string(appname_ptr);
    let cmdline = emu.maps.read_wide_string(cmdline_ptr);

    log_red!(emu, "kernel32!CreateProcessW '{}' '{}'", appname, cmdline);

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
