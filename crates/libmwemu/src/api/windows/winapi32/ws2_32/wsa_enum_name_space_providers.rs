use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn WSAEnumNameSpaceProvidersA(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers(emu, "WSAEnumNameSpaceProvidersA");
}

fn wsa_enum_name_space_providers(emu: &mut emu::Emu, api: &str) {
    let buf_len_ptr = WinApiAbi::arg(emu, 0);
    let buf_ptr = WinApiAbi::arg(emu, 1);
    log_red!(emu, "ws2_32!{api}");
    emu.set_last_error(10014);
    emu.regs_mut().rax = 0xFFFFFFFF;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn WSAEnumNameSpaceProvidersW(emu: &mut emu::Emu) {
    wsa_enum_name_space_providers(emu, "WSAEnumNameSpaceProvidersW");
}
