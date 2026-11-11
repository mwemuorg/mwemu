use crate::emu;

pub(super) fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "advapi32!StartServiceCtrlDispatcherA ptr:0x{:x}",
        service_table_entry_ptr
    );

    emu.regs_mut().rax = 1;
}

pub(super) fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.regs().rcx;

    log_red!(
        emu,
        "advapi32!StartServiceCtrlDispatcherW ptr:0x{:x}",
        service_table_entry_ptr
    );

    emu.regs_mut().rax = 1;
}
