use crate::emu;
use crate::windows::constants;

pub fn DeviceIoControlImplementation(emu: &mut emu::Emu) {
    let _hDevice = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _dwIoControlCode = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lpInBuffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _nInBufferSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _lpOutBuffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _nOutBufferSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");
    let _lpBytesReturned = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");
    let _lpOverlapped = emu
        .maps
        .read_dword(emu.regs().get_esp() + 28)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!DeviceIoControlImplementation");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..8 {
        emu.stack_pop32(false);
    }
}
