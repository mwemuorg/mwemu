use crate::emu;
use crate::windows::constants;

pub fn NtWow64CsrBaseCheckRunApp(emu: &mut emu::Emu) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _a2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _a3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _a4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _a5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _a6 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _a7 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let _a8 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
    let _a9 = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
    let _a10 = emu.maps.read_dword(emu.regs().get_esp() + 36).unwrap_or(0);

    log_red!(emu, "kernel32!NtWow64CsrBaseCheckRunApp");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..10 {
        emu.stack_pop32(false);
    }
}
