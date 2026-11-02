use crate::emu;

pub fn SetFileAttributesW(emu: &mut emu::Emu) {
    let path_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let attrs = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let path = if path_ptr != 0 {
        emu.maps.read_wide_string(path_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "kernel32!SetFileAttributesW '{}' attrs:0x{:x}",
        path,
        attrs
    );
    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
