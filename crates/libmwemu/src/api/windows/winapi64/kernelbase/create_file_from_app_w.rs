use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateFileFromAppW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let share = emu.regs().r8;
    let security = emu.regs().r9;
    let disposition = WinApiAbi::arg(emu, 4);
    let flags = WinApiAbi::arg(emu, 5);
    let template = WinApiAbi::arg(emu, 6);
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&format!("file_{}", path));
    log_red!(
        emu,
        "kernelbase!CreateFileFromAppW '{}' access:0x{:x} disp:{}, => 0x{:x}",
        path,
        access,
        disposition,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
