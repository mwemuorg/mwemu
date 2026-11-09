use crate::emu;

pub(super) fn crt_format_stub(emu: &mut emu::Emu, api: &str) {
    match api {
        "sprintf" | "snprintf" | "_snprintf" | "_sprintf_l" => {
            let buf = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
                emu.regs_mut().rax = fmt.len() as u64;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "fprintf" => {
            let stream = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!fprintf stream:0x{:x} fmt:'{}'", stream, fmt);
            emu.regs_mut().rax = fmt.len() as u64;
        }
        "sscanf" | "__stdio_common_vsscanf" => {
            let buf_ptr = emu.regs().rcx;
            let fmt_ptr = emu.regs().rdx;
            let buf = emu.maps.read_string(buf_ptr);
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!{} buf:'{}' fmt:'{}'", api, buf, fmt);
            emu.regs_mut().rax = 1;
        }
        "__stdio_common_vsprintf" | "__stdio_common_vsprintf_s" => {
            let _options = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let _buf_count = emu.regs().r8;
            let fmt_ptr = emu.regs().r9;
            let fmt = emu.maps.read_string(fmt_ptr);
            log_red!(emu, "wincrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
            if buf != 0 && !fmt.is_empty() {
                emu.maps.write_string(buf, &fmt);
            }
            emu.regs_mut().rax = fmt.len() as u64;
        }
        _ => {
            log_red!(emu, "wincrt!{}", api);
            emu.regs_mut().rax = 0;
        }
    }
}
