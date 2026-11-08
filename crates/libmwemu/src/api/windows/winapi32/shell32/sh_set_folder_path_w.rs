use crate::emu;

pub fn SHSetFolderPathW(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    log_red!(emu, "shell32!SHSetFolderPathW arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
