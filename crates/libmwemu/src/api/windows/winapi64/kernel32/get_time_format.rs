use crate::emu::Emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn GetTimeFormatA(emu: &mut Emu) {
    invoke(emu, false, "GetTimeFormatA");
}

pub fn GetTimeFormatW(emu: &mut Emu) {
    invoke(emu, true, "GetTimeFormatW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let output = WinApiAbi::arg(emu, 4);
    let capacity = WinApiAbi::arg(emu, 5) as i32;
    log_red!(
        emu,
        "kernel32!{} locale:0x{:x} flags:0x{:x}",
        api,
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = strings::write_text(emu, output, capacity, "00:00:00", wide);
}
