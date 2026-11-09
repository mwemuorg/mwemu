use crate::emu;
use crate::winapi::helper;

pub fn CreateFile2FromAppW(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let access = emu.regs().rdx;
    let share = emu.regs().r8;
    let security = emu.regs().r9;
    let disposition = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let template = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&format!("file_{}", path));
    log_red!(
        emu,
        "kernelbase!CreateFile2FromAppW '{}' access:0x{:x} disp:{}, => 0x{:x}",
        path,
        access,
        disposition,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
