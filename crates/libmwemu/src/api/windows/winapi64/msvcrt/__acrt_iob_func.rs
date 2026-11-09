use crate::emu;
use crate::winapi::helper;

pub fn __acrt_iob_func(emu: &mut emu::Emu) {
    let index = emu.regs().rcx as u32;
    let stream_name = match index {
        0 => "stdin",
        1 => "stdout",
        2 => "stderr",
        _ => "unknown_stream",
    };
    let hndl = helper::handler_create(stream_name);
    log_red!(
        emu,
        "msvcrt!__acrt_iob_func index:{} => 0x{:x}",
        index,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
