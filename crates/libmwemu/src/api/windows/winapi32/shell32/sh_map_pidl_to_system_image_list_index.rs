use crate::emu;
use crate::winapi::helper;

pub fn SHMapPIDLToSystemImageListIndex(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "shell32!SHMapPIDLToSystemImageListIndex arg1:0x{:x}",
        arg1
    );
    let handle = helper::handler_create("SHMapPIDLToSystemImageListIndex");
    emu.set_last_error(0);
    emu.regs_mut().rax = 2;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
