use crate::emu;

pub fn wsprintfW(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;
    let format_ptr = emu.regs().rdx;
    let format = emu.maps.read_wide_string(format_ptr);

    log_red!(
        emu,
        "user32!wsprintfW buffer=0x{:x} fmt='{}'",
        buffer,
        format
    );

    emu.maps.write_wide_string(buffer, &format);
    emu.regs_mut().rax = format.len() as u64;
}
