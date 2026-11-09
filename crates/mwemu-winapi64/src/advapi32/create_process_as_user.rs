use crate::emu;
use crate::winapi::helper;

pub fn CreateProcessAsUserA(emu: &mut emu::Emu) {
    let _token = emu.regs().rcx;
    let app_name_ptr = emu.regs().rdx;
    let cmd_line_ptr = emu.regs().r8;
    let _proc_attrs = emu.regs().r9;
    let _thread_attrs = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _inherit_handles = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _creation_flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _env = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _cur_dir = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let _startup_info = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let proc_info = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);
    let app = if app_name_ptr != 0 && app_name_ptr > 0xffff {
        emu.maps.read_string(app_name_ptr)
    } else {
        String::new()
    };
    let cmd = if cmd_line_ptr != 0 && cmd_line_ptr > 0xffff {
        emu.maps.read_string(cmd_line_ptr)
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
    log_red!(
        emu,
        "advapi32!CreateProcessAsUserA app:'{}' cmd:'{}'",
        app,
        cmd
    );
    emu.regs_mut().rax = 1;
}

pub fn CreateProcessAsUserW(emu: &mut emu::Emu) {
    let _token = emu.regs().rcx;
    let app_name_ptr = emu.regs().rdx;
    let cmd_line_ptr = emu.regs().r8;
    let _proc_attrs = emu.regs().r9;
    let _thread_attrs = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _inherit_handles = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _creation_flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _env = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _cur_dir = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let _startup_info = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let proc_info = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);
    let app = if app_name_ptr != 0 && app_name_ptr > 0xffff {
        emu.maps.read_wide_string(app_name_ptr)
    } else {
        String::new()
    };
    let cmd = if cmd_line_ptr != 0 && cmd_line_ptr > 0xffff {
        emu.maps.read_wide_string(cmd_line_ptr)
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
    log_red!(
        emu,
        "advapi32!CreateProcessAsUserW app:'{}' cmd:'{}'",
        app,
        cmd
    );
    emu.regs_mut().rax = 1;
}
