use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn LogonUserA(emu: &mut emu::Emu) {
    logon_user(emu, "LogonUserA", false);
}

fn logon_user(emu: &mut emu::Emu, api: &str, wide: bool) {
    let username_ptr = emu.regs().rcx;
    let domain_ptr = emu.regs().rdx;
    let _password_ptr = emu.regs().r8;
    let _logon_type = emu.regs().r9;
    let _logon_provider = WinApiAbi::arg(emu, 4);
    let token_ptr = WinApiAbi::arg(emu, 5);
    let user = if username_ptr != 0 && username_ptr > 0xffff {
        strings::read_text(emu, username_ptr, wide)
    } else {
        String::from("unknown")
    };
    let domain = if domain_ptr != 0 && domain_ptr > 0xffff {
        strings::read_text(emu, domain_ptr, wide)
    } else {
        String::from(".")
    };
    let token = helper::handler_create("logon_token");
    if token_ptr != 0 {
        emu.maps.write_qword(token_ptr, token);
    }
    log_red!(
        emu,
        "advapi32!{api} user:'{}' domain:'{}' => token:0x{:x}",
        user,
        domain,
        token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn LogonUserW(emu: &mut emu::Emu) {
    logon_user(emu, "LogonUserW", true);
}
