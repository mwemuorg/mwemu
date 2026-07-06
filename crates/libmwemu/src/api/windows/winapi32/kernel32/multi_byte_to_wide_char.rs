use crate::emu;

pub fn MultiByteToWideChar(emu: &mut emu::Emu) {
    let codepage = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!MultiByteToWideChar cannot read codepage");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!MultiByteToWideChar cannot read flags");
    let utf8_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!MultiByteToWideChar cannot read utf8_ptr") as u64;
    let cbMultiByte = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!MultiByteToWideChar cannot read cbMultiByte");
    let wide_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("kernel32!MultiByteToWideChar cannot read wide_ptr") as u64;
    let cchWideChar = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("kernel32!MultiByteToWideChar cannot read cchWideChar");

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
