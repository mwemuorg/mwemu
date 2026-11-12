use crate::emu;
use crate::winapi::helper;

pub fn setvbuf(emu: &mut emu::Emu) {
    let stream = emu.regs().rcx;
    let _buffer = emu.regs().rdx;
    let mode = emu.regs().r8 as u32;
    let size = emu.regs().r9 as u32;
    let uri = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!setvbuf stream:0x{:x}('{}') mode:{} size:{}",
        stream,
        uri,
        mode,
        size
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
