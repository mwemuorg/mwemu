use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptFindOIDInfo(emu: &mut emu::Emu) {
    let dw_key_type = WinApiAbi::arg(emu, 0);
    let pv_key = WinApiAbi::arg(emu, 1);
    let dw_group_id = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptFindOIDInfo dw_key_type:0x{:x} pv_key:0x{:x} dw_group_id:0x{:x}",
        dw_key_type,
        pv_key,
        dw_group_id
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
