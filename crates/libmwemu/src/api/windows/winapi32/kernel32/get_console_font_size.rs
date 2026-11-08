use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleFontSize(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let font_index = arg(emu, 1);
    log_red!(
        emu,
        "kernel32!GetConsoleFontSize h:0x{:x} font:{} => 8x16",
        h_console,
        font_index
    );
    let out_ptr = arg(emu, 1) as u64;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = (16u64 << 16) | 8;
    stdcall_ret(emu, 2);
}
