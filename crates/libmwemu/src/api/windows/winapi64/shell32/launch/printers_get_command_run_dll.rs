use crate::emu;
use crate::winapi::common::strings;

pub fn PrintersGetCommand_RunDLL(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(
        emu,
        "shell32!PrintersGetCommand_RunDLL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn PrintersGetCommand_RunDLLA(emu: &mut emu::Emu) {
    printers_get_command_run_dll(emu, "PrintersGetCommand_RunDLLA", false);
}

pub fn PrintersGetCommand_RunDLLW(emu: &mut emu::Emu) {
    printers_get_command_run_dll(emu, "PrintersGetCommand_RunDLLW", true);
}

fn printers_get_command_run_dll(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
        let arg1 = emu.regs().rcx;
        let arg2 = emu.regs().rdx as u32;
        log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    } else {
        let p0 = emu.regs().rcx;
        let p1 = emu.regs().rdx;
        let s = if p0 != 0 && p0 > 0xffff {
            strings::read_text(emu, p0, wide)
        } else {
            format!("0x{:x}", p0)
        };
        log_red!(emu, "shell32!{api} '{}'  p1:0x{:x}", s, p1);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
