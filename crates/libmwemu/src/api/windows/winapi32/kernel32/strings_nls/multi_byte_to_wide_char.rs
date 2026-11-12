use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn MultiByteToWideChar(emu: &mut emu::Emu) {
    let codepage = WinApiAbi::arg(emu, 0) as u32;
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let utf8_ptr = WinApiAbi::arg(emu, 2);
    let cbMultiByte = WinApiAbi::arg(emu, 3) as u32;
    let wide_ptr = WinApiAbi::arg(emu, 4);
    let cchWideChar = WinApiAbi::arg(emu, 5) as u32;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }

    let utf8 = emu.maps.read_string(utf8_ptr);
    let mut wide = String::new();
    for c in utf8.chars() {
        wide.push_str(&format!("{}", c));
        wide.push('\x00');
    }

    log_red!(
        emu,
        "kernel32!MultiByteToWideChar '{}' dst:0x{:x}",
        utf8,
        wide_ptr
    );

    if cchWideChar > 0 {
        emu.maps.write_string(wide_ptr, &wide);
    }
    emu.regs_mut().rax = wide.len() as u64;
}
