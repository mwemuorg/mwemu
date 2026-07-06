use crate::emu;
use crate::windows::constants;

pub fn TraceLoggingRegisterEx_EventRegister_EventSetInformation(emu: &mut emu::Emu) {
    let _hProvider = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _pEnableCallback = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _pCallbackContext = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");

    log_red!(
        emu,
        "kernel32!TraceLoggingRegisterEx_EventRegister_EventSetInformation"
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
