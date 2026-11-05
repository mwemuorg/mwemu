use crate::emu;
use crate::windows::constants;

pub fn GetNamedPipeHandleStateA(emu: &mut emu::Emu) {
    let hNamedPipe = emu.regs().rcx as u32;
    let _lpState = emu.regs().rdx as u32;
    let _lpCurInstances = emu.regs().r8 as u32;
    let _lpMaxCollectionCount = emu.regs().r9 as u32;
    let _lpCollectDataTimeout = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let lpUserName = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let nMaxUserNameSize = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let hNamedPipe = emu.maps.read_string(hNamedPipe as u64);
    let lpUserName = emu.maps.read_string(lpUserName);
    let nMaxUserNameSize = emu.maps.read_string(nMaxUserNameSize);

    log_red!(
        emu,
        "kernel32!GetNamedPipeHandleStateA {} {} {}",
        hNamedPipe,
        lpUserName,
        nMaxUserNameSize
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
