use crate::emu;

pub fn WSAGetServiceClassInfoW(emu: &mut emu::Emu) {
    let provider_id = emu.regs().rcx;
    let class_id = emu.regs().rdx;
    log_red!(emu, "ws2_32!WSAGetServiceClassInfoW");
    emu.set_last_error(10109);
    emu.regs_mut().rax = 0xFFFFFFFF;
}
