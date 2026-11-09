use crate::emu;
use crate::winapi::helper;

pub fn CreatePipe(emu: &mut emu::Emu) {
    let read_pipe = emu.regs().rcx;
    let write_pipe = emu.regs().rdx;
    log_red!(emu, "kernel32!CreatePipe");
    if read_pipe != 0 {
        emu.maps
            .write_qword(read_pipe, helper::handler_create("pipe_read"));
    }
    if write_pipe != 0 {
        emu.maps
            .write_qword(write_pipe, helper::handler_create("pipe_write"));
    }
    emu.regs_mut().rax = 1;
}
