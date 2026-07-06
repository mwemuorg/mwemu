use crate::emu;
use crate::winapi::winapi64::kernel32::load_library;

pub fn LoadLibraryExW(emu: &mut emu::Emu) {
    let dllptr = emu.regs().rcx;
    let dll = emu.maps.read_wide_string(dllptr);

    emu.regs_mut().rax = load_library(emu, &dll);

    log_red!(
        emu,
        "kernel32!LoadLibraryExW '{}' =0x{:x}",
        dll,
        emu.regs().rax
    );
}
