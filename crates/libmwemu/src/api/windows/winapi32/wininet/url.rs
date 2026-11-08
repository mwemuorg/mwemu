use crate::emu;

use super::{pop_stack32, read_stack_dword};

pub(super) fn internet_crack_url_a(emu: &mut emu::Emu) {
    let url_ptr =
        read_stack_dword(emu, 0, "wininet!InternetCrackUrlA error reading url_ptr") as u64;
    let _url_len = read_stack_dword(emu, 4, "wininet!InternetCrackUrlA error reading flags");
    let _flags = read_stack_dword(emu, 8, "wininet!InternetCrackUrlA error reading reserved");
    let _components =
        read_stack_dword(emu, 12, "wininet!InternetCrackUrlA error reading component");

    let url = emu.maps.read_string(url_ptr);

    log_red!(emu, "wininet!InternetCrackUrlA url: `{}`", url);

    pop_stack32(emu, 4);
    emu.regs_mut().rax = 1;
}

pub(super) fn internet_crack_url_w(emu: &mut emu::Emu) {
    let url_ptr =
        read_stack_dword(emu, 0, "wininet!InternetCrackUrlW error reading url_ptr") as u64;
    let _url_len = read_stack_dword(emu, 4, "wininet!InternetCrackUrlW error reading url_len");
    let _flags = read_stack_dword(emu, 8, "wininet!InternetCrackUrlW error reading flags");
    let _components = read_stack_dword(
        emu,
        12,
        "wininet!InternetCrackUrlW error reading components",
    );

    let url = emu.maps.read_wide_string(url_ptr);

    log_red!(emu, "wininet!InternetCrackUrlW url: `{}`", url);

    pop_stack32(emu, 4);
    emu.regs_mut().rax = 1;
}
