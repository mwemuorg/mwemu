use crate::emu;
use crate::windows::constants;

pub fn Heap32First(emu: &mut emu::Emu) {
    let _lphe = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _th32ProcessID = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _th32HeapID = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!Heap32First");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
