use crate::emu;

pub fn FlsSetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx;
    let val = emu.regs().rdx;

    log_red!(emu, "kernel32!FlsSetValue idx: {} val: 0x{:x}", idx, val);

    if emu.fls().len() > idx as usize {
        emu.fls_mut()[idx as usize] = val;
    } else {
        for _ in 0..=idx {
            emu.fls_mut().push(0);
        }
        emu.fls_mut()[idx as usize] = val;
    }

    emu.regs_mut().rax = 1;
}
