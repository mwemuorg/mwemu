use crate::emu;
use crate::windows::constants;

pub fn GetJapaneseEraItems(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!GetJapaneseEraItems");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
