use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn CopyContext(emu: &mut emu::Emu) {
    let dest = arg(emu, 0) as u64;
    let context_flags = arg(emu, 1) as u64;
    let src = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!CopyContext dest:0x{:x} flags:0x{:x} src:0x{:x}",
        dest,
        context_flags,
        src
    );

    if dest != 0 && src != 0 {
        let size = 1232usize;
        let data = emu.maps.read_bytes(src, size).to_vec();
        emu.maps.write_bytes(dest, &data);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
