use crate::emu;
use crate::windows::constants;

pub fn DeviceIoControlImplementation(emu: &mut emu::Emu) {
    let _hDevice = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwIoControlCode = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpInBuffer = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _nInBufferSize = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpOutBuffer = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _nOutBufferSize = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _lpBytesReturned = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let _lpOverlapped = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);

    log_red!(emu, "kernel32!DeviceIoControlImplementation");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
