use crate::api::windows::common::ntdll;
use crate::emu;
use scan_fmt::scan_fmt_some;

pub(super) fn dispatch(api: &str, emu: &mut emu::Emu) -> bool {
    match api {
        "stricmp" => stricmp(emu),
        "strlen" => strlen(emu),
        "strcat" => strcat(emu),
        "sscanf" => sscanf(emu),
        _ => return false,
    }
    true
}

fn stricmp(emu: &mut emu::Emu) {
    let str1ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!stricmp: error reading string1") as u64;
    let str2ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!stricmp: error reading string2") as u64;
    ntdll::stricmp(emu, str1ptr, str2ptr);

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

fn strlen(emu: &mut emu::Emu) {
    let s_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!strlen error reading string pointer") as u64;

    let len = ntdll::strlen(emu, s_ptr);
    emu.stack_pop32(false);
    emu.regs_mut().rax = len as u32 as u64;
}

fn strcat(emu: &mut emu::Emu) {
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!strcat error reading dst") as u64;
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!strcat error reading src") as u64;

    let dst_cont_ptr = ntdll::strcat(emu, dst_ptr, src_ptr);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.regs_mut().rax = dst_cont_ptr;
}

fn sscanf(emu: &mut emu::Emu) {
    let buffer_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("ntdll!sscanf error reading out buffer paramter") as u64;
    let fmt_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("ntdll!sscanf error reading format parameter") as u64;
    let list = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("ntdll!sscanf error reading list parameter");

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
    let b = buffer.as_str();
    let _params = scan_fmt_some!(b, &rust_fmt, i32);

    unimplemented!("sscanf is unimplemented for now.");
}
