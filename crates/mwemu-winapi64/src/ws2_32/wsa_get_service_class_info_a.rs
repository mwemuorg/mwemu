use crate::emu;

pub fn WSAGetServiceClassInfoA(emu: &mut emu::Emu) {
    let provider_id = emu.regs().rcx;
    let class_id = emu.regs().rdx;
    log_red!(emu, "ws2_32!WSAGetServiceClassInfoA");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
}
