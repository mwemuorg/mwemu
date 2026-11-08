use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn FillConsoleOutputAttribute(emu: &mut emu::Emu) {
    let h_console = arg(emu, 0) as u64;
    let attr = arg(emu, 1) as u64 as u16;
    let length = arg(emu, 2) as u64 as u32;
    let _coord = arg(emu, 3) as u64; // COORD packed as DWORD
    let attrs_written_ptr = arg(emu, 4) as u64;

    log_red!(
        emu,
        "kernel32!FillConsoleOutputAttribute h:0x{:x} attr:0x{:x} len:{}",
        h_console,
        attr,
        length
    );

    if attrs_written_ptr != 0 {
        emu.maps.write_dword(attrs_written_ptr, length);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
