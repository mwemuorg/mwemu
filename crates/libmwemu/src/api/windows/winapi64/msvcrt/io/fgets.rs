use crate::emu;
use crate::winapi::helper;

pub fn fgets(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let n = emu.regs().rdx as u32;
    let stream = emu.regs().r8;
    let name = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!fgets buf:0x{:x} n:{} stream:0x{:x} '{}'",
        buf,
        n,
        stream,
        name
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if buf != 0 && n > 0 {
        emu.maps.write_byte(buf, b'\n');
        if n > 1 {
            emu.maps.write_byte(buf + 1, 0);
        }
        emu.regs_mut().rax = buf;
    } else {
        emu.regs_mut().rax = 0;
    }
}
