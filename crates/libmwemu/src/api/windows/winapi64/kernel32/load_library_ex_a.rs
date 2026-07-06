use crate::emu;
use crate::winapi::winapi64::kernel32::load_library;

pub fn LoadLibraryExA(emu: &mut emu::Emu) {
    let dllptr = emu.regs().rcx;
    let dll = emu.maps.read_string(dllptr);

    emu.regs_mut().rax = load_library(emu, &dll);

    log_red!(
        emu,
        "kernel32!LoadLibraryExA  '{}' =0x{:x}",
        dll,
        emu.regs().rax
    );
}
