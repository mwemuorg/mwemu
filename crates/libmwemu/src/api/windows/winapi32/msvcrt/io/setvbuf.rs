use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn setvbuf(emu: &mut emu::Emu) {
    let stream = WinApiAbi::arg(emu, 0);
    let _buffer = WinApiAbi::arg(emu, 1) as u32;
    let mode = WinApiAbi::arg(emu, 2) as u32;
    let size = WinApiAbi::arg(emu, 3) as u32;
    let uri = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!setvbuf stream:0x{:x}('{}') mode:{} size:{}",
        stream,
        uri,
        mode,
        size
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // success
}
