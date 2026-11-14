use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn fwrite(emu: &mut emu::Emu) {
    let buff_ptr = WinApiAbi::arg(emu, 0);
    let size = WinApiAbi::arg(emu, 1) as u32;
    let nemb = WinApiAbi::arg(emu, 2) as u32;
    let file = WinApiAbi::arg(emu, 3) as u32;

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
