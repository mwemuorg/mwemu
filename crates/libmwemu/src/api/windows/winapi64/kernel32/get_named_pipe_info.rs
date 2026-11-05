use crate::emu;
use crate::winapi::helper;

pub fn GetNamedPipeInfo(emu: &mut emu::Emu) {
    let h_pipe = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    let out_buf_size_ptr = emu.regs().r8;
    let in_buf_size_ptr = emu.regs().r9;
    let max_instances_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(emu, "kernel32!GetNamedPipeInfo h:0x{:x}", h_pipe);

    if !helper::handler_exist(h_pipe) {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
        return;
    }

    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x00000001); // PIPE_SERVER_END
    }
    if out_buf_size_ptr != 0 {
        emu.maps.write_dword(out_buf_size_ptr, 4096);
    }
    if in_buf_size_ptr != 0 {
        emu.maps.write_dword(in_buf_size_ptr, 4096);
    }
    if max_instances_ptr != 0 {
        emu.maps.write_dword(max_instances_ptr, 1);
    }
    emu.regs_mut().rax = 1;
}
