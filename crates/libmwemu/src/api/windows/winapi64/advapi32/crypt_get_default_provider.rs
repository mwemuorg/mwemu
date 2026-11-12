use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGetDefaultProviderA(emu: &mut emu::Emu) {
    let _prov_type = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let name_ptr = emu.regs().r9;
    let name_len_ptr = WinApiAbi::arg(emu, 4);
    let name = "Microsoft Base Cryptographic Provider v1.0";
    if name_ptr != 0 {
        emu.maps.write_string(name_ptr, name);
    }
    if name_len_ptr != 0 {
        emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
    }
    log_red!(emu, "advapi32!CryptGetDefaultProviderA => '{}'", name);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CryptGetDefaultProviderW(emu: &mut emu::Emu) {
    let _prov_type = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let name_ptr = emu.regs().r9;
    let name_len_ptr = WinApiAbi::arg(emu, 4);
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
