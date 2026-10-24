use crate::emu;

pub fn FlsGetValue(emu: &mut emu::Emu) {
    let idx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

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
