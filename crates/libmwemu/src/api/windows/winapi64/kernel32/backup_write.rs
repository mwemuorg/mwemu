use crate::emu;
use crate::windows::constants;

pub fn BackupWrite(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let _lpBuffer = emu.regs().rdx as u32;
    let _nNumberOfBytesToWrite = emu.regs().r8 as u32;
    let _lpNumberOfBytesWritten = emu.regs().r9 as u32;
    let _bAbort = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _bProcessSecurity = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _lpContext = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    log_red!(emu, "kernel32!BackupWrite");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
