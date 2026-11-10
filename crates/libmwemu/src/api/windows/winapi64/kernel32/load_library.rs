use crate::emu;
use crate::winapi::winapi64::kernel32::load_library;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    let dllptr = emu.regs().rcx;
    let dll = emu.maps.read_string(dllptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = load_library(emu, &dll);

    if emu.cfg.verbose >= 1 {
        log_red!(
            emu,
            "** {} kernel32!LoadLibraryA  '{}' =0x{:x} rip: 0x{:x}",
            emu.pos,
            &dll,
            emu.regs().get_eax() as u32,
            emu.regs().rip
        );
    }
}

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    let dllptr = emu.regs().rcx;
    let dll = emu.maps.read_wide_string(dllptr);

    emu.set_last_error(0);
    emu.regs_mut().rax = load_library(emu, &dll);

    log_red!(
        emu,
        "** {} kernel32!LoadLibraryW  '{}' =0x{:x} rip: 0x{:x}",
        emu.pos,
        &dll,
        emu.regs().get_eax() as u32,
        emu.regs().rip
    );
}
