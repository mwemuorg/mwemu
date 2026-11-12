use crate::emu;

pub fn WSAEnumNameSpaceProvidersExA(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers_ex(emu, "WSAEnumNameSpaceProvidersExA");
}

fn wsa_enum_name_space_providers_ex(emu: &mut emu::Emu, api: &str) {
    let buf_len_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10014);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSAEnumNameSpaceProvidersExW(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers_ex(emu, "WSAEnumNameSpaceProvidersExW");
}
