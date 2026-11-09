use crate::emu;

pub fn IsThreadpoolTimerSet(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;

    log_red!(
        emu,
        "kernel32!IsThreadpoolTimerSet handle:0x{:x} p2:0x{:x}",
        handle,
        param2
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
