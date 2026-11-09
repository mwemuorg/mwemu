use crate::emu;

pub(super) fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    let service_table_entry_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "advapi321!StartServiceCtrlDispatcherW");

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}
