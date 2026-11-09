use crate::emu;

pub fn FlsSetValue(emu: &mut emu::Emu) {
    let idx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let val = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!FlsSetValue idx: {} val: {}", idx, val);

    if emu.fls().len() > idx as usize {
        emu.fls_mut()[idx as usize] = val as u64;
    } else {
        for _ in 0..=idx {
            emu.fls_mut().push(0);
        }
        emu.fls_mut()[idx as usize] = val as u64;
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
