use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn SHCreateShellItemArrayFromDataObject(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "shell32!SHCreateShellItemArrayFromDataObject arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shcreateshellitemarrayfromdataobject");
    emu.regs_mut().rax = handle;
    stdcall_ret(emu, 3);
}
