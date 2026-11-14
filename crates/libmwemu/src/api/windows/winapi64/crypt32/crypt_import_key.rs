use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptImportKey(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let data_ptr = emu.regs().rdx;
    let data_len = emu.regs().r8 as u32;
    let h_pub_key = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CryptImportKey prov: 0x{:x} data: 0x{:x} len: {} pubkey: 0x{:x}",
        h_prov,
        data_ptr,
        data_len,
        h_pub_key
    );
    // 5th arg: flags, 6th arg: phKey
    let h_key_ptr = WinApiAbi::arg(emu, 5);
    let h_key = helper::handler_create("importedkey");
    if h_key_ptr != 0 {
        emu.maps.write_qword(h_key_ptr, h_key);
    }
    emu.regs_mut().rax = 1; // TRUE
}
