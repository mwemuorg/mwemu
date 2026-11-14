use crate::emu;

pub(super) fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherA");
}

fn start_service_ctrl_dispatcher(emu: &mut emu::Emu, api: &str) {
    let service_table_entry_ptr = emu.regs().rcx;

    log_red!(emu, "advapi32!{api} ptr:0x{:x}", service_table_entry_ptr);

    emu.regs_mut().rax = 1;
}

pub(super) fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherW");
}
