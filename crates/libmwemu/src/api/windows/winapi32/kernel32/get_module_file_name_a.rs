use crate::emu;

pub fn GetModuleFileNameA(emu: &mut emu::Emu) {
    let hmod = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetModuleFileNameA: error reading param") as u64;
    let fname_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!GetModuleFileNameA: error reading param") as u64;
    let buff_sz = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!GetModuleFileNameA: error reading param");

    if buff_sz > 8 {
        emu.maps.write_string(fname_ptr, "c:\\test.exe");
    }

    log_red!(emu, "kernel32!GetModuleFileNameA 0x{:x}", hmod);

    for _ in 0..3 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 8;
}
