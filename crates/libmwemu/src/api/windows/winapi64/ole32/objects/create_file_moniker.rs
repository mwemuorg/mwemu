use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMoniker(emu: &mut emu::Emu) {
    let lpsz_path_name = emu.regs().rcx;
    let ppmk = emu.regs().rdx;
    let hndl = helper::handler_create("ole32://IMoniker");
    if ppmk != 0 {
        emu.maps.write_qword(ppmk, hndl);
    }
    let path = emu.maps.read_wide_string(lpsz_path_name);
    log_red!(emu, "ole32!CreateFileMoniker path:'{}' =0x{:x}", path, hndl);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
