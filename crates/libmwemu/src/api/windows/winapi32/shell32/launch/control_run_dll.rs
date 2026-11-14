use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn Control_RunDLL(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(
        emu,
        "shell32!Control_RunDLL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn Control_RunDLLA(emu: &mut emu::Emu) {
    control_run_dll(emu, "Control_RunDLLA");
}

fn control_run_dll(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn Control_RunDLLW(emu: &mut emu::Emu) {
    control_run_dll(emu, "Control_RunDLLW");
}
