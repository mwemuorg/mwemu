use crate::emu;
use crate::windows::constants;

pub fn CreateTapePartition(emu: &mut emu::Emu) {
    let _hDevice = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwPartitionMethod = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwCount = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwSize = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!CreateTapePartition");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
