use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATAdminCalcHashFromFileHandle(emu: &mut emu::Emu) {
    let h_file = WinApiAbi::arg(emu, 0);
    let pcb_hash = WinApiAbi::arg(emu, 1);
    let pb_hash = WinApiAbi::arg(emu, 2);
    let dw_flags = WinApiAbi::arg(emu, 3);
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle 0x{:x}",
        h_file
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
