use crate::emu;

pub fn GetCurrencyFormatEx(emu: &mut emu::Emu) {
    let _locale_ptr = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let value_ptr = emu.regs().r8;
    let _format_ptr = emu.regs().r9;
    let buf_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let buf_size = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        "0".to_string()
    };
    let formatted = format!("${}", value);
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, &formatted);
    }
    log_red!(
        emu,
        "kernelbase!GetCurrencyFormatEx '{}' => '{}'",
        value,
        formatted
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (formatted.len() + 1) as u64;
}
