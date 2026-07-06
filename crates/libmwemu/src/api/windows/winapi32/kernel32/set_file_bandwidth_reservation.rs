use crate::emu;
use crate::windows::constants;

pub fn SetFileBandwidthReservation(emu: &mut emu::Emu) {
    let _hFile = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _nPeriodMilliseconds = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _nBytesPerPeriod = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _bDiscardable = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _lpTransferSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _lpNumOutstandingRequests = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!SetFileBandwidthReservation");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
