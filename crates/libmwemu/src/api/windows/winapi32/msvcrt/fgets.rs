use crate::emu;
use crate::winapi::helper;

pub fn fgets(emu: &mut emu::Emu) {
    let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let stream = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let name = helper::handler_get_uri(stream);
    log_red!(
        emu,
        "msvcrt!fgets buf:0x{:x} n:{} stream:0x{:x} '{}'",
        buf,
        n,
        stream,
        name
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
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
