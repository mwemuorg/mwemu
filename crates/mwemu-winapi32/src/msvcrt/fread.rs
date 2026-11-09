use crate::emu;
use crate::winapi::helper;

pub fn fread(emu: &mut emu::Emu) {
    let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let fp = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name = helper::handler_get_uri(fp);
    log_red!(
        emu,
        "msvcrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} '{}'",
        buf,
        size,
        count,
        fp,
        name
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    helper::handler_set_eof(fp);
    emu.regs_mut().rax = 0;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
}
