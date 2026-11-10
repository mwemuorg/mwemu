use crate::emu;
use crate::winapi::common::{memory::Call, sections};
use crate::winapi::helper;

pub fn CloseHandle(emu: &mut emu::Emu) {
    let section_handle = Call::Api.arg(emu, 0);
    log_red!(emu, "kernel32!CloseHandle 0x{:X}", section_handle);
    if sections::close(emu, section_handle) {
        Call::Api.finish(emu, 1, 1);
        return;
    }
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    if !helper::handler_close(hndl) {
        log::trace!("\tinvalid handle.")
    }
    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
