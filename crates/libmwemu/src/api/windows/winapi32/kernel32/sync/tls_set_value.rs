use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn TlsSetValue(emu: &mut emu::Emu) {
    let idx = WinApiAbi::arg(emu, 0) as u32;
    let val = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!TlsSetValue idx: {} val: 0x{:x}", idx, val);

    if emu.tls32().len() > idx as usize {
        emu.tls32_mut()[idx as usize] = val;
    } else {
        for _ in 0..=idx {
            emu.tls32_mut().push(0);
        }
        emu.tls32_mut()[idx as usize] = val;
    }

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}
