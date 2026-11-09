use crate::emu;
use crate::winapi::helper;

pub fn setvbuf(emu: &mut emu::Emu) {
    let stream = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _buffer = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let mode = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let uri = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!setvbuf stream:0x{:x}('{}') mode:{} size:{}",
        stream,
        uri,
        mode,
        size
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success
}
