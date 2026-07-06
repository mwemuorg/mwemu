use crate::emu;
use crate::windows::constants;

pub fn MoveFileTransactedW(emu: &mut emu::Emu) {
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
    let _dwFlags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _hTransaction = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");

    let lpExistingFileName = emu.maps.read_wide_string(lpExistingFileName as u64);
    let lpNewFileName = emu.maps.read_wide_string(lpNewFileName as u64);

    log_red!(
        emu,
        "kernel32!MoveFileTransactedW {} {}",
        lpExistingFileName,
        lpNewFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
