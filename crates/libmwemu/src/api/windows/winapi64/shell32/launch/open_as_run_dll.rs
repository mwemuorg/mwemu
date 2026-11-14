use crate::emu;

pub fn OpenAs_RunDLL(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(
        emu,
        "shell32!OpenAs_RunDLL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn OpenAs_RunDLLA(emu: &mut emu::Emu) {
    open_as_run_dll(emu, "OpenAs_RunDLLA");
}

fn open_as_run_dll(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn OpenAs_RunDLLW(emu: &mut emu::Emu) {
    open_as_run_dll(emu, "OpenAs_RunDLLW");
}
