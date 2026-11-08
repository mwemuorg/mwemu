use crate::emu;
use crate::winapi::helper;

pub fn GetModuleHandleA(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let name = if name_ptr != 0 {
        emu.maps.read_string(name_ptr)
    } else {
        String::from("(null)")
    };
    let hndl = helper::handler_create(&format!("module_{}", name));
    log_red!(
        emu,
        "kernelbase!GetModuleHandleA '{}' => 0x{:x}",
        name,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
