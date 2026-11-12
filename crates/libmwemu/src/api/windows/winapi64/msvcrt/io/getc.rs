use crate::emu;
use crate::winapi::helper;

pub fn getc(emu: &mut emu::Emu, api: &str) {
    let stream = emu.regs().rcx;
    let name = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!{} stream:0x{:x} '{}' => EOF",
        api,
        stream,
        name
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0xffff_ffff;
}
