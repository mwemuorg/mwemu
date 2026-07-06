use crate::emu;
use crate::windows::constants;

pub fn api_DeviceIoControl(emu: &mut emu::Emu) {
    let hDevice = emu.regs().rcx;
    let dwIoControlCode = emu.regs().rdx;
    let lpInBuffer = emu.regs().r8;
    let lpOutBuffer = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let nOutBufferSize = emu
        .maps
        .read_qword(rsp + 0x20)
        .expect("DeviceIoControl arg6 stack error.");
    let lpBytesReturned = emu
        .maps
        .read_qword(rsp + 0x28)
        .expect("DeviceIoControl arg7 stack error.");

    log_red!(
        emu,
        "kernel32!DeviceIoControl hDev: 0x{:x} code: 0x{:x} buff: 0x{:x}",
        hDevice,
        dwIoControlCode,
        lpInBuffer
    );

    emu.regs_mut().rax = constants::TRUE;
}
