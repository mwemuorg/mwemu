use crate::emu;

pub fn SHInvokePrinterCommandA(emu: &mut emu::Emu) {
    sh_invoke_printer_command(emu, "SHInvokePrinterCommandA");
}

fn sh_invoke_printer_command(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHInvokePrinterCommandW(emu: &mut emu::Emu) {
    sh_invoke_printer_command(emu, "SHInvokePrinterCommandW");
}
