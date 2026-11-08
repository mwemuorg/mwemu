use crate::emu;
use crate::windows::constants;

pub fn GetVersion(emu: &mut emu::Emu) {
    emu.regs_mut().rax = constants::VERSION;
    log_red!(
        emu,
        "kernel32!GetVersion   =0x{:x}",
        emu.regs().get_eax() as u32
    );
}
