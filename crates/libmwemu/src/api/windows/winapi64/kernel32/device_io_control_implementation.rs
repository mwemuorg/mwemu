use crate::emu;
use crate::windows::constants;

pub fn DeviceIoControlImplementation(emu: &mut emu::Emu) {
    let _hDevice = emu.regs().rcx as u32;
    let _dwIoControlCode = emu.regs().rdx as u32;
    let _lpInBuffer = emu.regs().r8 as u32;
    let _nInBufferSize = emu.regs().r9 as u32;
    let _lpOutBuffer = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _nOutBufferSize = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _lpBytesReturned = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _lpOverlapped = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);

    log_red!(emu, "kernel32!DeviceIoControlImplementation");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
