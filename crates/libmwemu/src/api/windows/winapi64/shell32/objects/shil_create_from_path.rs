use crate::emu;
use crate::winapi::helper;

pub fn SHILCreateFromPath(emu: &mut emu::Emu) {
    let psz_path = emu.regs().rcx;
    let ppidl = emu.regs().rdx;
    let rgf_in_out = emu.regs().r8;
    let path = emu.maps.read_wide_string(psz_path);
    let hndl = helper::handler_create(&format!("pidl://{}", path));
    if ppidl != 0 {
        emu.maps.write_qword(ppidl, hndl);
    }
    log_red!(emu, "shell32!SHILCreateFromPath '{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
