use crate::emu;

pub fn CopyFileTransactedW(emu: &mut emu::Emu) {
    let lpExistingFileName = emu.regs().rcx as u32;
    let lpNewFileName = emu.regs().rdx as u32;
    let _lpProgressRoutine = emu.regs().r8 as u32;
    let _lpData = emu.regs().r9 as u32;
    let _pbCancel = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _dwCopyFlags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _hTransaction = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let lpExistingFileName = emu.maps.read_wide_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_wide_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!CopyFileTransactedW {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
