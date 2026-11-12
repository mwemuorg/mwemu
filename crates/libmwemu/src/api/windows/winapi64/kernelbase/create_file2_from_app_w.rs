use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateFile2FromAppW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let share = emu.regs().r8;
    let disposition = WinApiAbi::arg(emu, 3) as u32;
    let extended_parameters = WinApiAbi::arg(emu, 4);
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&format!("file_{}", path));
    log_red!(
        emu,
        "kernelbase!CreateFile2FromAppW '{}' access:0x{:x} disp:{} parameters:0x{:x} => 0x{:x}",
        path,
        access,
        disposition,
        extended_parameters,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
