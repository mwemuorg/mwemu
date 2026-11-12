use crate::emu;

pub fn WSAEnumNameSpaceProvidersExA(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers_ex(emu, "WSAEnumNameSpaceProvidersExA");
}

fn wsa_enum_name_space_providers_ex(emu: &mut emu::Emu, api: &str) {
    let buf_len_ptr = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10014);
    emu.regs_mut().rax = 0xFFFFFFFF;
}

pub fn WSAEnumNameSpaceProvidersExW(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers_ex(emu, "WSAEnumNameSpaceProvidersExW");
}
