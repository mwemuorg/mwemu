use crate::emu;

pub fn TlsSetValue(emu: &mut emu::Emu) {
    let idx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let val = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

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
