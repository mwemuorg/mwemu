use crate::emu;
use crate::windows::constants;

pub fn GetCompressedFileSizeTransactedW(emu: &mut emu::Emu) {
    let lpFileName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpFileSizeHigh = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _hTransaction = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!GetCompressedFileSizeTransactedW {}",
        lpFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
