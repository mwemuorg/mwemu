use crate::emu;

pub fn SHMapPIDLToSystemImageListIndex(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(
        emu,
        "shell32!SHMapPIDLToSystemImageListIndex arg1:0x{:x}",
        arg1
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
}
