use crate::emu;
use crate::windows::constants;

pub fn BackupWrite(emu: &mut emu::Emu) {
    let _hFile = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpBuffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _nNumberOfBytesToWrite = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _lpNumberOfBytesWritten = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _bAbort = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _bProcessSecurity = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");
    let _lpContext = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!BackupWrite");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
