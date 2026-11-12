use crate::emu;
use crate::winapi::common::strings;

pub fn ProcessIdleTasks(emu: &mut emu::Emu) {
    process_idle_tasks(emu, "ProcessIdleTasks", false);
}

pub fn ProcessIdleTasksW(emu: &mut emu::Emu) {
    process_idle_tasks(emu, "ProcessIdleTasksW", true);
}

fn process_idle_tasks(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
        let p0 = emu.regs().rcx;
        let p1 = emu.regs().rdx;
        let s = if p0 != 0 && p0 > 0xffff {
            strings::read_text(emu, p0, wide)
        } else {
            format!("0x{:x}", p0)
        };
        log_red!(emu, "advapi32!{api} '{}'  p1:0x{:x}", s, p1);
    } else {
        let handle = emu.regs().rcx;
        let arg2 = emu.regs().rdx;
        log_red!(emu, "advapi32!{api} h:0x{:x} arg2:0x{:x}", handle, arg2);
        let first_param = emu.regs().rcx;
        if first_param != 0 && emu.maps.is_mapped(first_param) {
            let _ = emu.maps.read_dword(first_param).unwrap_or(0);
        }
        for _ in 0..2 {}
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
