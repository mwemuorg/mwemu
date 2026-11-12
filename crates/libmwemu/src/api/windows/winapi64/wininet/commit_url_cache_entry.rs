use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CommitUrlCacheEntryA(emu: &mut emu::Emu) {
    commit_url_cache_entry(emu, "CommitUrlCacheEntryA");
}

fn commit_url_cache_entry(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = WinApiAbi::arg(emu, 4);
    let arg6 = WinApiAbi::arg(emu, 5);
    let arg7 = WinApiAbi::arg(emu, 6);
    let arg8 = WinApiAbi::arg(emu, 7);
    let arg9 = WinApiAbi::arg(emu, 8);
    log_red!(
        emu,
        "wininet!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn CommitUrlCacheEntryW(emu: &mut emu::Emu) {
    commit_url_cache_entry(emu, "CommitUrlCacheEntryW");
}
