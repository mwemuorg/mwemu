use crate::emu;

pub fn OpenAs_RunDLL(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "shell32!OpenAs_RunDLL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn OpenAs_RunDLLA(emu: &mut emu::Emu) {
    open_as_run_dll(emu, "OpenAs_RunDLLA");
}

fn open_as_run_dll(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn OpenAs_RunDLLW(emu: &mut emu::Emu) {
    open_as_run_dll(emu, "OpenAs_RunDLLW");
}
