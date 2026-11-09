use crate::emu;

pub fn GetBinaryType(emu: &mut emu::Emu) {
    let app_name_ptr = emu.regs().rcx;
    let binary_type_ptr = emu.regs().rdx;
    let app_name = emu.maps.read_string(app_name_ptr);
    log_red!(
        emu,
        "kernel32!GetBinaryType '{}' ptr:0x{:x}",
        app_name,
        binary_type_ptr
    );
    if binary_type_ptr != 0 {
        emu.maps.write_dword(binary_type_ptr, 6);
    }
    emu.regs_mut().rax = 1;
}
