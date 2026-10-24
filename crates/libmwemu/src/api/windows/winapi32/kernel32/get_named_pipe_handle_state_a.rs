use crate::emu;
use crate::windows::constants;

pub fn GetNamedPipeHandleStateA(emu: &mut emu::Emu) {
    let hNamedPipe = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpState = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpCurInstances = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpMaxCollectionCount = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpCollectDataTimeout = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let lpUserName = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let nMaxUserNameSize = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    let hNamedPipe = emu.maps.read_string(hNamedPipe as u64);
    let lpUserName = emu.maps.read_string(lpUserName as u64);
    let nMaxUserNameSize = emu.maps.read_string(nMaxUserNameSize as u64);

    log_red!(
        emu,
        "kernel32!GetNamedPipeHandleStateA {} {} {}",
        hNamedPipe,
        lpUserName,
        nMaxUserNameSize
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
