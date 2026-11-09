use crate::emu;

pub fn GetProcessMitigationPolicy(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let policy = emu.regs().rdx as u32;
    let buf_ptr = emu.regs().r8;
    let buf_len = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernel32!GetProcessMitigationPolicy h:0x{:x} policy:{} ptr:0x{:x} len:{}",
        h_process,
        policy,
        buf_ptr,
        buf_len
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf_ptr != 0 && buf_len > 0 {
        let clear_size = (buf_len as usize).min(64);
        for i in 0..clear_size {
            emu.maps.write_byte(buf_ptr + i as u64, 0);
        }
    }
    emu.regs_mut().rax = 1;
}
