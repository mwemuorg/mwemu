use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGetObjectUrl(emu: &mut emu::Emu) {
    let psz_url_oid = WinApiAbi::arg(emu, 0);
    let pv_para = WinApiAbi::arg(emu, 1);
    let dw_flags = WinApiAbi::arg(emu, 2);
    let p_url_array = WinApiAbi::arg(emu, 3);
    let pcb_url_array = WinApiAbi::arg(emu, 4);
    let p_url_info = WinApiAbi::arg(emu, 5);
    let pcb_url_info = WinApiAbi::arg(emu, 6);
    let pv_reserved = WinApiAbi::arg(emu, 7);
    log_red!(emu, "crypt32!CryptGetObjectUrl 0x{:x}", psz_url_oid);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
