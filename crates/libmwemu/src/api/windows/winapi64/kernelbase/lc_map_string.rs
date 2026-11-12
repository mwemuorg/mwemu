use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn LCMapStringA(emu: &mut emu::Emu) {
    lc_map_string(emu, "LCMapStringA", false);
}

pub fn LCMapStringW(emu: &mut emu::Emu) {
    lc_map_string(emu, "LCMapStringW", true);
}

fn lc_map_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _locale = emu.regs().rcx as u32;
    let flags = emu.regs().rdx as u32;
    let src_ptr = emu.regs().r8;
    if wide {
        let src_len = emu.regs().r9 as i32;
        let dst_ptr = WinApiAbi::arg(emu, 4);
        let dst_len = WinApiAbi::arg(emu, 5) as i32;
        let src = if src_ptr != 0 {
            strings::read_text(emu, src_ptr, wide)
        } else {
            String::new()
        };
        let mapped = if flags & 0x00000100 != 0 {
            src.to_uppercase()
        } else if flags & 0x00000200 != 0 {
            src.to_lowercase()
        } else {
            src.clone()
        };
        let result_len = mapped.encode_utf16().count() + 1;
        if dst_ptr != 0 && dst_len > 0 {
            strings::write_string(emu, dst_ptr, &mapped, wide);
        }
        log_red!(
            emu,
            "kernelbase!{api} flags:0x{:x} '{}' => '{}'",
            flags,
            src,
            mapped
        );
        emu.set_last_error(0);
        emu.regs_mut().rax = result_len as u64;
    } else {
        let _src_len = emu.regs().r9 as i32;
        let dst_ptr = WinApiAbi::arg(emu, 4);
        let dst_len = WinApiAbi::arg(emu, 5) as i32;
        let src = if src_ptr != 0 {
            strings::read_text(emu, src_ptr, wide)
        } else {
            String::new()
        };
        let mapped = if flags & 0x00000100 != 0 {
            src.to_uppercase()
        } else if flags & 0x00000200 != 0 {
            src.to_lowercase()
        } else {
            src.clone()
        };
        let result_len = mapped.len() + 1;
        if dst_ptr != 0 && dst_len > 0 {
            strings::write_string(emu, dst_ptr, &mapped, wide);
        }
        log_red!(
            emu,
            "kernelbase!{api} flags:0x{:x} '{}' => '{}'",
            flags,
            src,
            mapped
        );
        emu.set_last_error(0);
        emu.regs_mut().rax = result_len as u64;
    }
}
