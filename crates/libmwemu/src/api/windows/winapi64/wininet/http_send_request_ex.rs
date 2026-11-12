use crate::emu;

pub fn HttpSendRequestExA(emu: &mut emu::Emu) {
    http_send_request_ex(emu, "HttpSendRequestExA");
}

fn http_send_request_ex(emu: &mut emu::Emu, api: &str) {
    let handle = emu.regs().rcx;
    let buffers_in = emu.regs().rdx;
    let buffers_out = emu.regs().r8;
    let flags = emu.regs().r9;
    log_red!(
        emu,
        "wininet!{api} handle:0x{:x} buf_in:0x{:x} flags:0x{:x}",
        handle,
        buffers_in,
        flags
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn HttpSendRequestExW(emu: &mut emu::Emu) {
    http_send_request_ex(emu, "HttpSendRequestExW");
}
