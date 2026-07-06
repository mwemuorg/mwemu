use crate::emu;

pub fn DeleteFileA(emu: &mut emu::Emu) {
    let filename_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!DeleteFileA cannot read filename_ptr") as u64;

    let filename = emu.maps.read_string(filename_ptr);

    log_red!(
        emu,
        "kernel32!DeleteFileA `{}` (stub: not deleting)",
        filename
    );

    emu.stack_pop32(false);

    emu.regs_mut().rax = 1; // Success
}
