use crate::emu;
use crate::windows::constants;

pub fn FindNextFileA(emu: &mut emu::Emu) {
    let hndl = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FindNextFileA cannot read the handle");
    let find_data = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!FindNextFileA cannot read the find_data");

    log_red!(emu, "kernel32!FindNextFileA");

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = constants::ERROR_NO_MORE_FILES;
}
