use crate::emu;

pub fn FlsSetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx;
    let val = emu.regs().rdx as u32;

    log_red!(emu, "kernel32!FlsSetValue idx: {} val: {}", idx, val);

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
