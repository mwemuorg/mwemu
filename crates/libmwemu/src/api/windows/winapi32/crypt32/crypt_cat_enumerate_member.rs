use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATEnumerateMember(emu: &mut emu::Emu) {
    let h_catalog = WinApiAbi::arg(emu, 0);
    let p_prev_member = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "crypt32!CryptCATEnumerateMember h_catalog:0x{:x} p_prev_member:0x{:x}",
        h_catalog,
        p_prev_member
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
