use crate::emu::Emu;
use crate::winapi::common::strings;

pub fn lstrcpy(emu: &mut Emu) {
    copy(emu, false, "lstrcpy");
}

pub fn lstrcpyA(emu: &mut Emu) {
    copy(emu, false, "lstrcpyA");
}

pub fn lstrcpyW(emu: &mut Emu) {
    copy(emu, true, "lstrcpyW");
}

fn copy(emu: &mut Emu, wide: bool, api: &str) {
    let destination = emu.regs().rcx;
    let source = emu.regs().rdx;
    let text = strings::read_text(emu, source, wide);
    let bytes = strings::encode_text(&format!("{text}\0"), wide);
    log_red!(
        emu,
        "kernel32!{} destination:0x{:x} source:0x{:x}",
        api,
        destination,
        source
    );
    emu.regs_mut().rax = if destination != 0
        && source != 0
        && emu
            .maps
            .validate_write_range(destination, bytes.len() as u64)
        && emu.maps.write_bytes(destination, &bytes)
    {
        destination
    } else {
        0
    };
}
