use crate::emu;

pub fn CryptGetDefaultProviderW(emu: &mut emu::Emu) {
    let _prov_type = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let name_ptr = emu.regs().r9;
    let name_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let name = "Microsoft Base Cryptographic Provider v1.0";
    if name_ptr != 0 {
        emu.maps.write_wide_string(name_ptr, name);
    }
    if name_len_ptr != 0 {
        emu.maps
            .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
    }
    log_red!(emu, "advapi32!CryptGetDefaultProviderW => '{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
