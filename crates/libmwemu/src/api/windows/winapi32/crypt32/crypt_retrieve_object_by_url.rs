use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptRetrieveObjectByUrl(emu: &mut emu::Emu) {
    let psz_url = WinApiAbi::arg(emu, 0);
    let psz_object_oid = WinApiAbi::arg(emu, 1);
    let dw_retrieve_flags = WinApiAbi::arg(emu, 2);
    let dw_timeout = WinApiAbi::arg(emu, 3);
    let ppv_object = WinApiAbi::arg(emu, 4);
    let h_async_retrieve = WinApiAbi::arg(emu, 5);
    let p_credentials = WinApiAbi::arg(emu, 6);
    log_red!(emu, "crypt32!CryptRetrieveObjectByUrl 0x{:x}", psz_url);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
