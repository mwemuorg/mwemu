use crate::emu;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    let dllptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dll = emu.maps.read_string(dllptr);

    emu.regs_mut().rax = load_library(emu, &dll);

    log_red!(
        emu,
        "** {} kernel32!LoadLibraryA  '{}' =0x{:x} rip: 0x{:x}",
        emu.pos,
        &dll,
        emu.regs().get_eax() as u32,
        emu.regs().rip
    );

    emu.stack_pop32(false);

    //TODO: instead returning the base, return a handle that have linked the dll name
}

pub fn LoadLibraryW(emu: &mut emu::Emu) {
    let dllptr = match emu.maps.read_dword(emu.regs().get_esp()) {
        Some(v) => v as u64,
        None => {
            log::warn!("bad LoadLibraryW parameter");
            return;
        }
    };
    let dll = emu.maps.read_wide_string(dllptr);
    log_red!(emu, "LoadLibraryW  '{}'", dll);

    //if dll == "ntdll.dll" {
    //  emu.regs_mut().rax = emu.maps.get_mem("ntdll").get_base();
    //}

    emu.regs_mut().rax = load_library(emu, &dll);

    emu.stack_pop32(false);
}
