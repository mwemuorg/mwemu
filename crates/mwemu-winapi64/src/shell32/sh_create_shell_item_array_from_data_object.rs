use crate::emu;
use crate::winapi::helper;

pub fn SHCreateShellItemArrayFromDataObject(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHCreateShellItemArrayFromDataObject arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shcreateshellitemarrayfromdataobject");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
