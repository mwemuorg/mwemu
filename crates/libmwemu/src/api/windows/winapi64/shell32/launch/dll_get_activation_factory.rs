use crate::emu;

pub fn DllGetActivationFactory(emu: &mut emu::Emu) {
    let class_id_ptr = emu.regs().rcx;
    let iid_ptr = emu.regs().rdx;
    let factory_ptr = emu.regs().r8;
    log_red!(emu, "shell32!DllGetActivationFactory");
    emu.set_last_error(0);
    emu.regs_mut().rax = 0x80004001u64;
}
