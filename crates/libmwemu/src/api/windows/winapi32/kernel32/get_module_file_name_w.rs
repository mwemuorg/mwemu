use crate::emu;

pub fn GetModuleFileNameW(emu: &mut emu::Emu) {
    let hmodule = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!GetModuleFileNameW cannot read hmodule");
    let out_filename_ptr =
        emu.maps
            .read_dword(emu.regs().get_esp() + 4)
            .expect("kernel32!GetModuleFileNameW cannot read out_filename_ptr") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!GetModuleFileNameW cannot read out_filename_ptr");

    log_red!(emu, "kernel32!GetModuleFileNameW");

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.maps.write_wide_string(out_filename_ptr, "jowei3r.exe");
    emu.regs_mut().rax = 11;
}
