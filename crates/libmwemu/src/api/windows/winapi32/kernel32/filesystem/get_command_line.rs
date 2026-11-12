use crate::emu;
use crate::winapi::common::strings;

pub fn GetCommandLineA(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineA", false);
}

pub fn GetCommandLineW(emu: &mut emu::Emu) {
    get_command_line(emu, "GetCommandLineW", true);
}

fn get_command_line(emu: &mut emu::Emu, api: &str, wide: bool) {
    if wide {
        log_red!(emu, "kernel32!GetCommandlineW");
    } else {
        log_red!(emu, "kernel32!GetCommandlineA");
    }

    let addr = match emu.heap_mut().allocate(2048) {
        Some(a) => a,
        None => {
            log::error!("kernel32!{api}: out of memory");
            emu.regs_mut().rax = 0;
            return;
        }
    };
    strings::write_string(emu, addr, "test.exe", wide);
    emu.regs_mut().rax = addr;
}
