use crate::emu;
use crate::winapi::helper;

pub fn FindResourceExW(emu: &mut emu::Emu) {
    let module = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let type_ptr = emu.regs().r8;
    let name = if name_ptr > 0xffff {
        emu.maps.read_wide_string(name_ptr)
    } else {
        format!("#{}", name_ptr)
    };
    let hndl = helper::handler_create(&format!("resource_{}", name));
    log_red!(
        emu,
        "kernelbase!FindResourceExW module:0x{:x} '{}' => 0x{:x}",
        module,
        name,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
