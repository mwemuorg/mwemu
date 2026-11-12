use crate::emu;

pub fn _except_handler4(emu: &mut emu::Emu, api: &str) {
    let exc_record = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let frame_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "msvcrt!{} exc:0x{:x} frame:0x{:x}",
        api,
        exc_record,
        frame_ptr
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // ExceptionContinueSearch
}
