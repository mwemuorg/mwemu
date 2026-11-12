use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptSIPCreateIndirectData(emu: &mut emu::Emu) {
    let p_subject_info = WinApiAbi::arg(emu, 0);
    let pcb_indirect_data = WinApiAbi::arg(emu, 1);
    let p_indirect_data = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptSIPCreateIndirectData p_subject_info:0x{:x} pcb_indirect_data:0x{:x} p_indirect_data:0x{:x}",
        p_subject_info,
        pcb_indirect_data,
        p_indirect_data
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
