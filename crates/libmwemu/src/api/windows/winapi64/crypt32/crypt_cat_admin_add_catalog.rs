use crate::emu;
use crate::winapi::helper;

pub fn CryptCATAdminAddCatalog(emu: &mut emu::Emu) {
    let admin = emu.regs().rcx;
    let catalog_file = emu.regs().rdx;
    log_red!(emu, "crypt32!CryptCATAdminAddCatalog admin:0x{:x}", admin);
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptcatadmincatalog");
    emu.regs_mut().rax = handle;
}
