use crate::emu;

pub fn DosPathToSessionPathA(emu: &mut emu::Emu) {
    let session_id = emu.regs().rcx;
    let p_in_path = emu.regs().rdx;
    let pp_out_path = emu.regs().r8;

    let path = emu.maps.read_string(p_in_path);
    log_red!(
        emu,
        "kernel32!DosPathToSessionPathA session:{} path:'{}'",
        session_id,
        path
    );

    if pp_out_path != 0 {
        emu.maps.write_qword(pp_out_path, p_in_path);
    }
    emu.regs_mut().rax = 1;
}
