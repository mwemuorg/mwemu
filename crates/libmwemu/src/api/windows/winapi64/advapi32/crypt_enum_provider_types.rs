use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptEnumProviderTypesA(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let prov_type_ptr = emu.regs().r9;
    let name_ptr = WinApiAbi::arg(emu, 4);
    let name_len_ptr = WinApiAbi::arg(emu, 5);
    log_red!(emu, "advapi32!CryptEnumProviderTypesA index:{}", index);
    if index == 0 {
        if prov_type_ptr != 0 {
            emu.maps.write_dword(prov_type_ptr, 1);
        }
        let name = "RSA Full (Signature and Key Exchange)";
        if name_ptr != 0 {
            emu.maps.write_string(name_ptr, name);
        }
        if name_len_ptr != 0 {
            emu.maps.write_dword(name_len_ptr, name.len() as u32 + 1);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

pub fn CryptEnumProviderTypesW(emu: &mut emu::Emu) {
    let index = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let prov_type_ptr = emu.regs().r9;
    let name_ptr = WinApiAbi::arg(emu, 4);
    let name_len_ptr = WinApiAbi::arg(emu, 5);
    log_red!(emu, "advapi32!CryptEnumProviderTypesW index:{}", index);
    if index == 0 {
        if prov_type_ptr != 0 {
            emu.maps.write_dword(prov_type_ptr, 1);
        }
        let name = "RSA Full (Signature and Key Exchange)";
        if name_ptr != 0 {
            emu.maps.write_wide_string(name_ptr, name);
        }
        if name_len_ptr != 0 {
            emu.maps
                .write_dword(name_len_ptr, (name.len() as u32 + 1) * 2);
        }
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
