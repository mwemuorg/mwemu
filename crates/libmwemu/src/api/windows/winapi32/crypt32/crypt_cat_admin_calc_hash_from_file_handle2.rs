use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATAdminCalcHashFromFileHandle2(emu: &mut emu::Emu) {
    let h_cat_admin = WinApiAbi::arg(emu, 0);
    let h_file = WinApiAbi::arg(emu, 1);
    let pcb_hash = WinApiAbi::arg(emu, 2);
    let pb_hash = WinApiAbi::arg(emu, 3);
    let dw_flags = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle2 0x{:x}",
        h_cat_admin
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
