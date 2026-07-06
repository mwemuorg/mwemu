use crate::emu;
use crate::windows::constants;

pub fn GetProfileSectionW(emu: &mut emu::Emu) {
    let lpAppName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpReturnedString = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _nSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpAppName = emu.maps.read_wide_string(lpAppName as u64);
    let lpReturnedString = emu.maps.read_wide_string(lpReturnedString as u64);

    log_red!(
        emu,
        "kernel32!GetProfileSectionW {} {}",
        lpAppName,
        lpReturnedString
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
