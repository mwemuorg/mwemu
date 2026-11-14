use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FindNLSStringEx(emu: &mut emu::Emu) {
    let locale_ptr = emu.regs().rcx;
    let flags = emu.regs().rdx as u32;
    let src_ptr = emu.regs().r8;
    let src_count = emu.regs().r9 as i32;
    let value_ptr = WinApiAbi::arg(emu, 4);
    let locale = if locale_ptr != 0 && locale_ptr > 0xffff {
        emu.maps.read_wide_string(locale_ptr)
    } else {
        String::new()
    };
    let src = emu.maps.read_wide_string(src_ptr);
    let value = if value_ptr != 0 {
        emu.maps.read_wide_string(value_ptr)
    } else {
        String::new()
    };
    let result = if let Some(pos) = src.to_lowercase().find(&value.to_lowercase()) {
        pos as i64
    } else {
        -1i64
    };
    log_red!(
        emu,
        "kernelbase!FindNLSStringEx locale:'{}' src:'{}' find:'{}' => {}",
        locale,
        src,
        value,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
