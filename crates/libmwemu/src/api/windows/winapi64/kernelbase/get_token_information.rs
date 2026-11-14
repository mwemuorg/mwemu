use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_handle = WinApiAbi::arg(emu, 0);
    let token_information_class = WinApiAbi::arg(emu, 1);
    let token_information = emu.regs().r8;
    let token_information_length = emu.regs().r9;
    let return_length = WinApiAbi::arg(emu, 4);

    log_red!(
        emu,
        "kernelbase!GetTokenInformation token_information_class: 0x{:x}",
        token_information_class
    );

    emu.regs_mut().rax = 1;
}
