use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FindStringOrdinal(emu: &mut emu::Emu) {
    let flags = emu.regs().rcx as u32;
    let src_ptr = emu.regs().rdx;
    let src_len = emu.regs().r8 as i32;
    let value_ptr = emu.regs().r9;
    let value_len = WinApiAbi::arg(emu, 4) as i32;
    let ignore_case = WinApiAbi::arg(emu, 5) != 0;
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    let result = if ignore_case {
        src.to_lowercase().find(&value.to_lowercase())
    } else {
        src.find(&value)
    };
    let idx = result.map(|p| p as i64).unwrap_or(-1);
    log_red!(
        emu,
        "kernelbase!FindStringOrdinal '{}' in '{}' => {}",
        value,
        src,
        idx
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = idx as u64;
}
