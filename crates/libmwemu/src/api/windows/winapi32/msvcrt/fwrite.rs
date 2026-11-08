use crate::emu;
use crate::winapi::helper;

pub(super) fn fwrite(emu: &mut emu::Emu) {
    let buff_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let nemb = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let file = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let filename = helper::handler_get_uri(file as u64);
    let total = (size as u64) * (nemb as u64);

    log_red!(
        emu,
        "msvcrt!fwrite `{}` buf:0x{:x} size:{} count:{}",
        filename,
        buff_ptr,
        size,
        nemb
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = nemb as u64;
}
