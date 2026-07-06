use crate::emu;
use crate::windows::constants;

pub fn GetBinaryTypeW(emu: &mut emu::Emu) {
    let lpApplicationName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpBinaryType = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpApplicationName = emu.maps.read_wide_string(lpApplicationName as u64);

    log_red!(emu, "kernel32!GetBinaryTypeW {}", lpApplicationName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
