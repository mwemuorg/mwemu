use crate::emu;

pub fn __CppXcptFilter(emu: &mut emu::Emu) {
    let exc_num = emu.regs().rcx;
    let exc_ptrs = emu.regs().rdx;
    if exc_ptrs != 0 {
        let exc_record = emu.maps.read_qword(exc_ptrs).unwrap_or(0);
        log_red!(
            emu,
            "msvcrt!__CppXcptFilter exc:0x{:x} ptrs:0x{:x} rec:0x{:x}",
            exc_num,
            exc_ptrs,
            exc_record
        );
        let first_param = emu.regs().rcx;
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
    } else {
        log_red!(emu, "msvcrt!__CppXcptFilter exc:0x{:x} ptrs:NULL", exc_num);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // EXCEPTION_EXECUTE_HANDLER
}
