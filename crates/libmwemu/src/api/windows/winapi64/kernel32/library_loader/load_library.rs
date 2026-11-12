use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi64::kernel32::load_library;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    load_library_text(emu, "LoadLibraryA", false);
}

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    load_library_text(emu, "LoadLibraryW", true);
}

fn load_library_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let dllptr = emu.regs().rcx;
    let dll = strings::read_text(emu, dllptr, wide);
    emu.set_last_error(0);
    emu.regs_mut().rax = load_library(emu, &dll);
    if wide {
        log_red!(
            emu,
            "** {} kernel32!{api}  '{}' =0x{:x} rip: 0x{:x}",
            emu.pos,
            &dll,
            emu.regs().get_eax() as u32,
            emu.regs().rip
        );
    } else {
        if emu.cfg.verbose >= 1 {
            log_red!(
                emu,
                "** {} kernel32!{api}  '{}' =0x{:x} rip: 0x{:x}",
                emu.pos,
                &dll,
                emu.regs().get_eax() as u32,
                emu.regs().rip
            );
        }
    }
}
