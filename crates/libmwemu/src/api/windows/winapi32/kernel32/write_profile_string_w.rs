use crate::emu;
use crate::windows::constants;

pub fn WriteProfileStringW(emu: &mut emu::Emu) {
    let lpAppName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpKeyName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let lpString = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpAppName = emu.maps.read_wide_string(lpAppName as u64);
    let lpKeyName = emu.maps.read_wide_string(lpKeyName as u64);
    let lpString = emu.maps.read_wide_string(lpString as u64);

    log_red!(
        emu,
        "kernel32!WriteProfileStringW {} {} {}",
        lpAppName,
        lpKeyName,
        lpString
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
