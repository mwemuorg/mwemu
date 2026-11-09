use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessA(emu: &mut emu::Emu) {
    let app_name_ptr = emu.regs().rcx;
    let cmd_line_ptr = emu.regs().rdx;
    let proc_attrs = emu.regs().r8;
    let thread_attrs = emu.regs().r9;
    let inherit = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let env = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let cur_dir_ptr = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let startup_info = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let proc_info = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);
    let app = if app_name_ptr != 0 {
        emu.maps.read_string(app_name_ptr)
    } else {
        String::new()
    };
    let cmd = if cmd_line_ptr != 0 {
        emu.maps.read_string(cmd_line_ptr)
    } else {
        String::new()
    };
    if proc_info != 0 {
        let hndl = helper::handler_create(&format!("process_{}", app));
        let thread_hndl = helper::handler_create("process_thread");
        emu.maps.write_dword(proc_info + 8, 0x1000); // dwProcessId
        emu.maps.write_dword(proc_info + 12, 0x2000); // dwThreadId
    }
    log_red!(emu, "kernelbase!CreateProcessA app:'{}' cmd:'{}'", app, cmd);
    emu.regs_mut().rax = 1;
}
