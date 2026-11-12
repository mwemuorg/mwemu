use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn DeviceIoControlImplementation(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwIoControlCode = WinApiAbi::arg(emu, 1) as u32;
    let _lpInBuffer = WinApiAbi::arg(emu, 2) as u32;
    let _nInBufferSize = WinApiAbi::arg(emu, 3) as u32;
    let _lpOutBuffer = WinApiAbi::arg(emu, 4) as u32;
    let _nOutBufferSize = WinApiAbi::arg(emu, 5) as u32;
    let _lpBytesReturned = WinApiAbi::arg(emu, 6) as u32;
    let _lpOverlapped = WinApiAbi::arg(emu, 7) as u32;

    log_red!(emu, "kernel32!DeviceIoControlImplementation");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
