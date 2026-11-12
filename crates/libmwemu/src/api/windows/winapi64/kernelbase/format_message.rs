use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn FormatMessageA(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageA", false);
}

fn format_message(emu: &mut emu::Emu, api: &str, wide: bool) {
    let flags = emu.regs().rcx;
    let source = emu.regs().rdx;
    let msg_id = emu.regs().r8;
    let lang_id = emu.regs().r9;
    let buf_ptr = WinApiAbi::arg(emu, 4);
    let buf_size = WinApiAbi::arg(emu, 5);
    let msg = format!("Error {}", msg_id);
    if buf_ptr != 0 && buf_size > 0 {
        strings::write_string(emu, buf_ptr, &msg, wide);
    }
    log_red!(
        emu,
        "kernelbase!{api} flags:0x{:x} msg_id:{} => '{}'",
        flags,
        msg_id,
        msg
    );
    let src = emu.regs().rcx;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = strings::read_text(emu, src, wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = msg.len() as u64;
}

pub fn FormatMessageW(emu: &mut emu::Emu) {
    format_message(emu, "FormatMessageW", true);
}
