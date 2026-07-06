use crate::emu;
use crate::winapi::winapi32::kernel32::load_library;

pub fn LoadLibraryA(emu: &mut emu::Emu) {
    let dllptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("bad LoadLibraryA parameter") as u64;
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
