use crate::emu;

pub fn _except_handler4(emu: &mut emu::Emu, api: &str) {
    let exc_record = emu.regs().rcx;
    let frame_ptr = emu.regs().rdx;
    let exc_code = if exc_record != 0 {
        emu.maps.read_dword(exc_record).unwrap_or(0)
    } else {
        0
    };
    log_red!(
        emu,
        "msvcrt!{} exc:0x{:x} code:0x{:x} frame:0x{:x}",
        api,
        exc_record,
        exc_code,
        frame_ptr
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
