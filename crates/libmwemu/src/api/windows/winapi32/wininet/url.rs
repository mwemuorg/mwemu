use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub(super) fn internet_crack_url_a(emu: &mut emu::Emu) {
    internet_crack_url(emu, "InternetCrackUrlA", false);
}

fn internet_crack_url(emu: &mut emu::Emu, api: &str, wide: bool) {
    let url_ptr = WinApiAbi::arg(emu, 0);
    let _url_len = WinApiAbi::arg(emu, 1) as u32;
    let _flags = WinApiAbi::arg(emu, 2) as u32;
    let _components = WinApiAbi::arg(emu, 3) as u32;

    let url = strings::read_text(emu, url_ptr, wide);

    log_red!(emu, "wininet!{api} url: `{}`", url);

    WinApiAbi::stdcall_ret(emu, 4);
    emu.regs_mut().rax = 1;
}

pub(super) fn internet_crack_url_w(emu: &mut emu::Emu) {
    internet_crack_url(emu, "InternetCrackUrlW", true);
}
