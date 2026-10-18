use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IsWow64Process(emu: &mut emu::Emu) {
    let out_ptr = arg(emu, 1) as u64;
    log_red!(emu, "kernel32!IsWow64Process out: 0x{:x}", out_ptr);
    // A 32-bit process on the 64-bit Windows the emulator models is WOW64.
    emu.maps.write_dword(out_ptr, 1);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
