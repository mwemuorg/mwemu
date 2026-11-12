use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATCDFOpen(emu: &mut emu::Emu) {
    let pwsz_file_path = WinApiAbi::arg(emu, 0);
    let pfn_parse_error = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "crypt32!CryptCATCDFOpen pwsz_file_path:0x{:x} pfn_parse_error:0x{:x}",
        pwsz_file_path,
        pfn_parse_error
    );
    let hndl = helper::handler_create("CryptCATCDFOpen");
    emu.regs_mut().rax = hndl;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
