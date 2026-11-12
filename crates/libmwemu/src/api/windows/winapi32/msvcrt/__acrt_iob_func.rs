use crate::emu;
use crate::winapi::helper;

pub fn __acrt_iob_func(emu: &mut emu::Emu) {
    let index = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
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

    emu.regs_mut().rax = hndl;
}
