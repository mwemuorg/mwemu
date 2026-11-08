use crate::emu;
use crate::winapi::helper;

pub fn LogonUserA(emu: &mut emu::Emu) {
    let username_ptr = emu.regs().rcx;
    let domain_ptr = emu.regs().rdx;
    let _password_ptr = emu.regs().r8;
    let _logon_type = emu.regs().r9;
    let _logon_provider = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let token_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let user = if username_ptr != 0 && username_ptr > 0xffff {
        emu.maps.read_string(username_ptr)
    } else {
        String::from("unknown")
    };
    let domain = if domain_ptr != 0 && domain_ptr > 0xffff {
        emu.maps.read_string(domain_ptr)
    } else {
        String::from(".")
    };
    let token = helper::handler_create("logon_token");
    if token_ptr != 0 {
        emu.maps.write_qword(token_ptr, token);
    }
    log_red!(
        emu,
        "advapi32!LogonUserA user:'{}' domain:'{}' => token:0x{:x}",
        user,
        domain,
        token
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
