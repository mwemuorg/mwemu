use crate::emu;
use crate::winapi::helper;

pub fn RegisterServiceCtrlHandlerExA(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegisterServiceCtrlHandlerExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerExA");
    emu.regs_mut().rax = hndl;
}

pub fn RegisterServiceCtrlHandlerExW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(
        emu,
        "advapi32!RegisterServiceCtrlHandlerExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerExW");
    emu.regs_mut().rax = hndl;
}
