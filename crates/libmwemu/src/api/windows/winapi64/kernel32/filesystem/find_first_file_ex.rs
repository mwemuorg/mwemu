use crate::emu::Emu;
use crate::winapi::winapi64::kernel32::{FindFirstFileA, FindFirstFileW};

pub fn FindFirstFileExA(emu: &mut Emu) {
    invoke(emu, false, "FindFirstFileExA");
}

pub fn FindFirstFileExW(emu: &mut Emu) {
    invoke(emu, true, "FindFirstFileExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    log_red!(
        emu,
        "kernel32!{} pattern:0x{:x} data:0x{:x}",
        api,
        emu.regs().rcx,
        emu.regs().r8
    );
    emu.regs_mut().rdx = emu.regs().r8;
    if wide {
        FindFirstFileW(emu);
    } else {
        FindFirstFileA(emu);
    }
}
