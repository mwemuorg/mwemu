use crate::emu;

pub fn DeleteProcThreadAttributeList(emu: &mut emu::Emu) {
    let attr_list = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!DeleteProcThreadAttributeList list:0x{:x}",
        attr_list
    );
    emu.set_last_error(0);
}
