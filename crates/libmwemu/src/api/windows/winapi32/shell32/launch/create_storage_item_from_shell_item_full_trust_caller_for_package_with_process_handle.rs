use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle(
    emu: &mut emu::Emu,
) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!CreateStorageItemFromShellItem_FullTrustCaller_ForPackage_WithProcessHandle arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create(
        "createstorageitemfromshellitem_fulltrustcaller_forpackage_withprocesshandle",
    );
    emu.regs_mut().rax = handle;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
