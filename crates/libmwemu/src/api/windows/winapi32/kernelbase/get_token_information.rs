use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetTokenInformation(emu: &mut emu::Emu) {
    let token_information_class = arg(emu, 1);
    log_red!(
        emu,
        "kernelbase!GetTokenInformation class: 0x{:x}",
        token_information_class
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
