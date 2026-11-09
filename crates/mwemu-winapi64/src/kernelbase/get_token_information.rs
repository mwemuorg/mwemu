use crate::emu;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = emu.regs().rdx;
    let token_information_class = emu.regs().rcx;
    let token_information = emu.regs().r8;
    let token_information_length = emu.regs().r9;
    let return_length = emu.maps.read_qword(emu.regs().rsp + 0x20);

    log_red!(
        emu,
        "kernelbase!GetTokenInformation token_information_class: 0x{:x}",
        token_information_class
    );

    emu.regs_mut().rax = 1;
}
