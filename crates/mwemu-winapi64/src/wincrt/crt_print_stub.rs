use crate::emu;

pub(super) fn crt_print_stub(emu: &mut emu::Emu, api: &str) {
    let fmt_ptr = emu.regs().rcx;
    let fmt = if api == "wprintf" || api == "vwprintf" {
        emu.maps.read_wide_string(fmt_ptr)
    } else {
        emu.maps.read_string(fmt_ptr)
    };
    log_red!(emu, "wincrt!{} '{}'", api, fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
