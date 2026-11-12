use crate::emu;
use crate::winapi::helper;

pub fn CreateFontA(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontA");
}

fn create_font(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..14u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create(&format!("gdi32://{api}"));
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}

pub fn CreateFontW(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontW");
}
