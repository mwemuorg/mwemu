use crate::emu;

pub fn _except_handler3(emu: &mut emu::Emu, api: &str) {
    let exception_record = emu.regs().rcx;
    let exc_code = if exception_record != 0 {
        emu.maps.read_dword(exception_record).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!{} exc_record:0x{:x} code:0x{:x}",
        api,
        exception_record,
        exc_code
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
