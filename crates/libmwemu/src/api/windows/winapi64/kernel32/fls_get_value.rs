use crate::emu;

pub fn FlsGetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx;
    if idx as usize > emu.fls().len() {
        emu.regs_mut().rax = 0;
    } else {
        emu.regs_mut().rax = emu.fls()[idx as usize] as u64;
    }

    log_red!(
        emu,
        "kernel32!FlsGetValue idx: {} =0x{:x}",
        idx,
        emu.regs().get_eax() as u32
    );
}
