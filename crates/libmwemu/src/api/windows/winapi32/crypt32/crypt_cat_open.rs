use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATOpen(emu: &mut emu::Emu) {
    let pwsz_file_name = WinApiAbi::arg(emu, 0);
    let fdw_open_flags = WinApiAbi::arg(emu, 1);
    let h_prov = WinApiAbi::arg(emu, 2);
    let dw_public_version = WinApiAbi::arg(emu, 3);
    let dw_encoded_msg_type = WinApiAbi::arg(emu, 4);
    log_red!(emu, "crypt32!CryptCATOpen 0x{:x}", pwsz_file_name);
    let hndl = helper::handler_create("CryptCATOpen");
    emu.regs_mut().rax = hndl;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
