use super::map_virtual_key;
use crate::emu;

pub(super) fn MapVirtualKeyA(emu: &mut emu::Emu) {
    map_key(emu, "MapVirtualKeyA");
}

fn map_key(emu: &mut emu::Emu, api: &str) {
    let code = emu.regs().rcx as u32;
    let map_type = emu.regs().rdx as u32;
    let result = map_virtual_key(code, map_type);
    log_red!(
        emu,
        "user32!{api} code:0x{:x} type:{} => 0x{:x}",
        code,
        map_type,
        result
    );
    emu.regs_mut().rax = result;
}

pub(super) fn MapVirtualKeyW(emu: &mut emu::Emu) {
    map_key(emu, "MapVirtualKeyW");
}
