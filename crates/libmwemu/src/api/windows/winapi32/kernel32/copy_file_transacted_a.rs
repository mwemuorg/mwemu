use crate::emu;
use crate::windows::constants;

pub fn CopyFileTransactedA(emu: &mut emu::Emu) {
    let lpExistingFileName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpNewFileName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lpProgressRoutine = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _lpData = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _pbCancel = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _dwCopyFlags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");
    let _hTransaction = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");

    let lpExistingFileName = emu.maps.read_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!CopyFileTransactedA {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
