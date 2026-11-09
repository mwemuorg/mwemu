use crate::emu;
use crate::winapi::helper;

pub fn CreateUmsCompletionList(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "kernel32!CreateUmsCompletionList arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    let nparams = emu.banzai.get_params("CreateUmsCompletionList");
    if nparams > 0 {
        for _ in 0..nparams {
            emu.stack_pop32(false);
        }
    }
    let hndl = helper::handler_create("kernel32://CreateUmsCompletionList");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}
