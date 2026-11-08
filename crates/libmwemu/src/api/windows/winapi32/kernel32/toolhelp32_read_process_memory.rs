use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn Toolhelp32ReadProcessMemory(emu: &mut emu::Emu) {
    let process = arg(emu, 0) as u64;
    let base_addr = arg(emu, 1) as u64;
    let buffer = arg(emu, 2) as u64;
    let size = arg(emu, 3) as u64 as usize;

    log_red!(
        emu,
        "kernel32!Toolhelp32ReadProcessMemory proc:0x{:x} addr:0x{:x} size:{}",
        process,
        base_addr,
        size
    );

    if buffer != 0 && base_addr != 0 && size > 0 && size <= 0x100000 {
        let data = emu.maps.read_bytes(base_addr, size).to_vec();
        emu.maps.write_bytes(buffer, &data);
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 4);
}
