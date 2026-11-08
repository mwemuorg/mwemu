use crate::emu;

pub fn GetTimeFormatW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let _time = emu.regs().r8;
    let _fmt = emu.regs().r9;
    let buf = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buf_len = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let time_str = "00:00:00";
    log_red!(
        emu,
        "kernel32!GetTimeFormatW locale:{} flags:0x{:x}",
        locale,
        flags
    );
    if buf != 0 && buf_len > 0 {
        emu.maps.write_wide_string(buf, time_str);
    }
    emu.regs_mut().rax = (time_str.len() + 1) as u64;
}
