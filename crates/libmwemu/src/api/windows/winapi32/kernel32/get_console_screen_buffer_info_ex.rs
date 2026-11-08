use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleScreenBufferInfoEx(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let info_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetConsoleScreenBufferInfoEx h:0x{:x} ptr:0x{:x}",
        h_console,
        info_ptr
    );
    if info_ptr != 0 {
        emu.maps.write_dword(info_ptr, 96);
        emu.maps.write_word(info_ptr + 4, 80);
        emu.maps.write_word(info_ptr + 6, 25);
        emu.maps.write_word(info_ptr + 8, 0);
        emu.maps.write_word(info_ptr + 10, 0);
        emu.maps.write_word(info_ptr + 12, 7);
        emu.maps.write_word(info_ptr + 14, 0);
        emu.maps.write_word(info_ptr + 16, 0);
        emu.maps.write_word(info_ptr + 18, 79);
        emu.maps.write_word(info_ptr + 20, 24);
        emu.maps.write_word(info_ptr + 22, 80);
        emu.maps.write_word(info_ptr + 24, 25);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
