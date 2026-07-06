use crate::emu;
use crate::windows::constants;

pub fn DefineDosDeviceA(emu: &mut emu::Emu) {
    let _dwFlags = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let lpDeviceName = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let lpTargetPath = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    let lpDeviceName = emu.maps.read_string(lpDeviceName as u64);
    let lpTargetPath = emu.maps.read_string(lpTargetPath as u64);

    log_red!(
        emu,
        "kernel32!DefineDosDeviceA {} {}",
        lpDeviceName,
        lpTargetPath
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
