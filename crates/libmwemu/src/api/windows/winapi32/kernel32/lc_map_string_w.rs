use crate::emu;

pub fn LCMapStringW(emu: &mut emu::Emu) {
    let locale = emu
        .maps
        .read_dword(emu.regs().rsp)
        .expect("kernel32!LCMapStringW error reading param");
    let flags = emu
        .maps
        .read_dword(emu.regs().rsp + 4)
        .expect("kernel32!LCMapStringW error reading param");
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 8)
        .expect("kernel32!LCMapStringW error reading param") as u64;
    let src_sz = emu
        .maps
        .read_dword(emu.regs().rsp + 12)
        .expect("kernel32!LCMapStringW error reading param");
    let dest_ptr = emu
        .maps
        .read_dword(emu.regs().rsp + 16)
        .expect("kernel32!LCMapStringW error reading param") as u64;
    let dest_sz = emu
        .maps
        .read_dword(emu.regs().rsp + 20)
        .expect("kernel32!LCMapStringW error reading param");

    let s = emu.maps.read_wide_string(src_ptr);

    log_red!(
        emu,
        "kernel32!LCMapStringW `{}` dst:0x{:x} sz:{}->{}",
        s,
        dest_ptr,
        src_sz,
        dest_sz
    );

    if dest_ptr > 0 {
        emu.maps.write_wide_string(dest_ptr, &s);
    }

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
