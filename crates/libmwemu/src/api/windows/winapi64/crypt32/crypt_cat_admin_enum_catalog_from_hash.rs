use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminEnumCatalogFromHash(emu: &mut emu::Emu) {
    let admin = emu.regs().rcx;
    let hash_ptr = emu.regs().rdx;
    let hash_len = emu.regs().r8 as u32;
    log_red!(
        emu,
        "crypt32!CryptCATAdminEnumCatalogFromHash admin:0x{:x}",
        admin
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptcatenumcatalog");
    emu.regs_mut().rax = handle;
}
