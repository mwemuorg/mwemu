use crate::emu;
use crate::winapi::helper;

pub fn CryptDuplicateHash(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let ph_hash = emu.regs().r9;
    let new_handle = helper::handler_create("crypt_hash_dup");
    log_red!(
        emu,
        "advapi32!CryptDuplicateHash src:0x{:x} => 0x{:x}",
        hhash,
        new_handle
    );
    if ph_hash != 0 {
        emu.maps.write_qword(ph_hash, new_handle);
    }
    emu.regs_mut().rax = 1;
}
