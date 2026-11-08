use crate::emu;
use crate::winapi::helper;

pub fn CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage(
    emu: &mut emu::Emu,
) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!CreateStorageItemFromShellItem_FullTrustCaller_UseImplicitFlagsAndPackage arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create(
        "createstorageitemfromshellitem_fulltrustcaller_useimplicitflagsandpackage",
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
