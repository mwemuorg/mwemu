use crate::emu;
use crate::winapi::helper;

pub fn CryptCATCDFOpen(emu: &mut emu::Emu) {
    let pwsz_file_path = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pfn_parse_error = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
