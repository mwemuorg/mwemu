use crate::emu;
use crate::winapi::helper;

pub fn CreateStorageItemFromPath_FullTrustCaller_ForPackage(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!CreateStorageItemFromPath_FullTrustCaller_ForPackage arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("createstorageitemfrompath_fulltrustcaller_forpackage");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
