use crate::emu;
use crate::winapi::helper;

pub fn OpenEventLogA(emu: &mut emu::Emu) {
    open_event_log(emu, "OpenEventLogA");
}

fn open_event_log(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "advapi32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create(&format!("advapi32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn OpenEventLogW(emu: &mut emu::Emu) {
    open_event_log(emu, "OpenEventLogW");
}
