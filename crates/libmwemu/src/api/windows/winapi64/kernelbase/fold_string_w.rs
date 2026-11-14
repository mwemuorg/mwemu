use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn FoldStringW(emu: &mut emu::Emu) {
    let _flags = emu.regs().rcx as u32;
    let src_ptr = emu.regs().rdx;
    let src_len = emu.regs().r8 as i32;
    let dst_ptr = emu.regs().r9;
    let dst_len = WinApiAbi::arg(emu, 4) as i32;
    let src = if src_ptr != 0 {
        emu.maps.read_wide_string(src_ptr)
    } else {
        String::new()
    };
    let result_len = src.encode_utf16().count() + 1;
    if dst_ptr != 0 && dst_len > 0 {
        emu.maps.write_wide_string(dst_ptr, &src);
    }
    log_red!(emu, "kernelbase!FoldStringW '{}' => {}", src, result_len);
    emu.set_last_error(0);
    emu.regs_mut().rax = result_len as u64;
}
