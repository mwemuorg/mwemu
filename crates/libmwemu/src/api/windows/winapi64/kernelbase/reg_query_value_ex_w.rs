use crate::emu;

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let data_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let size_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let name = emu.maps.read_wide_string(name_ptr);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1);
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    log_red!(
        emu,
        "kernelbase!RegQueryValueExW hkey:0x{:x} '{}'",
        hkey,
        name
    );
    emu.regs_mut().rax = 2;
}
