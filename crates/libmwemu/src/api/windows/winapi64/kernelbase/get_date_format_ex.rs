use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetDateFormatEx(emu: &mut emu::Emu) {
    let _locale_ptr = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let _date_ptr = emu.regs().r8;
    let _format_ptr = emu.regs().r9;
    let buf_ptr = WinApiAbi::arg(emu, 4);
    let buf_size = WinApiAbi::arg(emu, 5) as i32;
    let date = "11/01/2026";
    let written = strings::write_text(emu, buf_ptr, buf_size, date, true);
    log_red!(emu, "kernelbase!GetDateFormatEx => '{}'", date);

    emu.regs_mut().rax = written;
}
