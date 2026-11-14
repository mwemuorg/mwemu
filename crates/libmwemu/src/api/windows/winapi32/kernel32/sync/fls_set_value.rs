use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FlsSetValue(emu: &mut emu::Emu) {
    let idx = WinApiAbi::arg(emu, 0) as u32;
    let val = WinApiAbi::arg(emu, 1) as u32;

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
