use crate::emu;

pub fn _CxxThrowException(emu: &mut emu::Emu) {
    let exc_object = emu.regs().rcx;
    let throw_info = emu.regs().rdx;
    let vtable = if exc_object != 0 {
        emu.maps.read_qword(exc_object).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!_CxxThrowException obj:0x{:x} vtable:0x{:x} info:0x{:x}",
        exc_object,
        vtable,
        throw_info
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
