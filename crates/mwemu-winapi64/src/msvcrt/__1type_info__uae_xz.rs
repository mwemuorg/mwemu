use crate::emu;

pub fn __1type_info__UAE_XZ(emu: &mut emu::Emu) {
    let this_ptr = emu.regs().rcx;
    let vtable = if this_ptr != 0 {
        emu.maps.read_qword(this_ptr).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!~type_info this:0x{:x} vtable:0x{:x}",
        this_ptr,
        vtable
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
}
