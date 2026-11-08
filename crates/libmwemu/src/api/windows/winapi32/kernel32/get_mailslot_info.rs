use crate::emu;
use crate::windows::constants;

pub fn GetMailslotInfo(emu: &mut emu::Emu) {
    let _hMailslot = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpMaxMessageSize = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpNextSize = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _lpMessageCount = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpReadTimeout = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!GetMailslotInfo");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
