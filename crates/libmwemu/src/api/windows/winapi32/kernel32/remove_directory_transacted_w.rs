use crate::emu;
use crate::windows::constants;

pub fn RemoveDirectoryTransactedW(emu: &mut emu::Emu) {
    let lpPathName = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _hTransaction = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");

    let lpPathName = emu.maps.read_wide_string(lpPathName as u64);

    log_red!(emu, "kernel32!RemoveDirectoryTransactedW {}", lpPathName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
