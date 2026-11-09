use crate::emu;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx;
    let source = emu.regs().rdx;
    let msg_id = emu.regs().r8;
    let lang_id = emu.regs().r9;
    let buf_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let msg = format!("Error {}", msg_id);
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_string(buf_ptr, &msg);
    }
    log_red!(
        emu,
        "kernelbase!FormatMessageA flags:0x{:x} msg_id:{} => '{}'",
        flags,
        msg_id,
        msg
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = msg.len() as u64;
}
