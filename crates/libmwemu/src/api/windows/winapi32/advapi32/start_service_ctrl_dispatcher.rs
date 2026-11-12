use crate::emu;

pub(super) fn StartServiceCtrlDispatcherA(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherA");
}

pub(super) fn StartServiceCtrlDispatcherW(emu: &mut emu::Emu) {
    start_service_ctrl_dispatcher(emu, "StartServiceCtrlDispatcherW");
}

fn start_service_ctrl_dispatcher(emu: &mut emu::Emu, api: &str) {
    let service_table_entry_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    /*
    let service_name = emu.maps.read_dword(service_table_entry_ptr as u64)
        .unwrap_or(0);
    let service_name = emu.maps.read_dword((service_table_entry_ptr+4) as u64)
        .unwrap_or(0);*/

    log_red!(emu, "advapi32!{api}");

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}
