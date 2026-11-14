use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ConvertSecurityDescriptorToStringSecurityDescriptorA(emu: &mut emu::Emu) {
    convert_security_descriptor_to_string_security_descriptor(
        emu,
        "ConvertSecurityDescriptorToStringSecurityDescriptorA",
        false,
    );
}

fn convert_security_descriptor_to_string_security_descriptor(
    emu: &mut emu::Emu,
    api: &str,
    wide: bool,
) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
    let p4 = WinApiAbi::arg(emu, 4);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn ConvertSecurityDescriptorToStringSecurityDescriptorW(emu: &mut emu::Emu) {
    convert_security_descriptor_to_string_security_descriptor(
        emu,
        "ConvertSecurityDescriptorToStringSecurityDescriptorW",
        true,
    );
}
