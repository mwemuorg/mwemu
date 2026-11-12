use crate::emu;
use crate::winapi::common::strings;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineA", false);
}

fn get_command_line(emu: &mut emu::Emu, api: &str, wide: bool) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(emu, "kernel32!{api}");

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!{api}: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };

    let exe_name = emu.cfg.exe_name.clone();
    strings::write_string(emu, addr, &exe_name, wide);
    emu.regs_mut().rax = addr;
}

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineW", true);
}
