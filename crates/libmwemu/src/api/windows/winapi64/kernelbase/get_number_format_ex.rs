use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn GetNumberFormatEx(emu: &mut emu::Emu) {
    let _locale_ptr = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let value_ptr = emu.regs().r8;
    let _format_ptr = emu.regs().r9;
    let buf_ptr = WinApiAbi::arg(emu, 4);
    let buf_size = WinApiAbi::arg(emu, 5) as i32;
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        "0".to_string()
    };
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, &value);
    }
    log_red!(
        emu,
        "kernelbase!GetNumberFormatEx '{}' => '{}'",
        value,
        value
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = (value.len() + 1) as u64;
}
