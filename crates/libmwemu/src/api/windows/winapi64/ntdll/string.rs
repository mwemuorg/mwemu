use crate::api::windows::common::ntdll;
use crate::emu;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "stricmp" => stricmp(emu),
        "strlen" => strlen(emu),
        "sscanf" => sscanf(emu),
        _ => return false,
    }
    true
}

fn stricmp(emu: &mut emu::Emu) {
    let str1ptr = emu.regs().rcx;
    let str2ptr = emu.regs().rdx;
    ntdll::stricmp(emu, str1ptr, str2ptr);
}

fn strlen(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx as usize;
    log_red!(emu, "** {} ntdll!strlen {:x}", emu.pos, s_ptr);

    if s_ptr == 0 {
        emu.regs_mut().rax = 0;
        return;
    }

    let s = emu.maps.read_string(s_ptr as u64);
    let l = s.len();

    log_red!(emu, "ntdll!strlen: `{}` {}", s, l);

    emu.regs_mut().rax = l as u32 as u64;
}

fn sscanf(emu: &mut emu::Emu) {
    let buffer_ptr = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let list = emu.regs().r8;

    let buffer = emu.maps.read_string(buffer_ptr);
    let fmt = emu.maps.read_string(fmt_ptr);

    log_red!(emu, "ntdll!sscanf out_buff: `{}` fmt: `{}`", buffer, fmt);

    let rust_fmt = fmt
        .replace("%x", "{x}")
        .replace("%d", "{}")
        .replace("%s", "{}")
        .replace("%hu", "{u16}")
        .replace("%i", "{}")
        .replace("%o", "{o}")
        .replace("%f", "{}");
    let params = rust_fmt.matches("{").count();

    unimplemented!("sscanf is unimplemented for now.");
}
