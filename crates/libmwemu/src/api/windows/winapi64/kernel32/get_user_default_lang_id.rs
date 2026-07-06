use crate::emu;

pub fn GetUserDefaultLangId(emu: &mut emu::Emu) {
    emu.regs_mut().rax = 0x000000000000ffff;
    log_red!(
        emu,
        "kernel32!GetUserDefaultLangID =0x{:x}",
        emu.regs().rax as u16
    );
}
