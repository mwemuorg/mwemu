use crate::emu;
use crate::winapi::helper;
pub fn LoadEnclaveData(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let lp_address = emu.regs().rdx;
    let lp_buffer = emu.regs().r8;
    let n_size = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!LoadEnclaveData hProc:0x{:x} addr:0x{:x} buf:0x{:x} size:{}",
        h_process,
        lp_address,
        lp_buffer,
        n_size
    );
    let handle = helper::handler_create("LoadEnclaveData");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
