use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNamedPipeInfo(emu: &mut emu::Emu) {
    let h_pipe = arg(emu, 0) as u64;
    let flags_ptr = arg(emu, 1) as u64;
    let out_buf_size_ptr = arg(emu, 2) as u64;
    let in_buf_size_ptr = arg(emu, 3) as u64;
    let max_instances_ptr = arg(emu, 4) as u64;

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
    stdcall_ret(emu, 5);
}
