use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FlsGetValue(emu: &mut emu::Emu) {
    let idx = WinApiAbi::arg(emu, 0) as u32;

    emu.stack_pop32(false);

    if idx as usize > emu.fls().len() {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = emu.fls_mut()[idx as usize];
    }

    log_red!(
        emu,
        "kernel32!FlsGetValue idx: {} =0x{:x}",
        idx,
        emu.regs().get_eax() as u32
    );
}
