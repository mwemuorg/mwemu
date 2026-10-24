use crate::emu;
use crate::windows::constants;

pub fn EnumerateLocalComputerNamesA(emu: &mut emu::Emu) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpMultiByteStr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _a3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    let lpMultiByteStr = emu.maps.read_string(lpMultiByteStr as u64);

    log_red!(
        emu,
        "kernel32!EnumerateLocalComputerNamesA {}",
        lpMultiByteStr
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
