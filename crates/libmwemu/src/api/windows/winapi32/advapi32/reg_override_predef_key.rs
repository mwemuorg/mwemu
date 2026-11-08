use crate::emu;

pub fn RegOverridePredefKey(emu: &mut emu::Emu) {
    let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "advapi32!RegOverridePredefKey h:0x{:x} arg2:0x{:x}",
        handle,
        arg2
    );
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
