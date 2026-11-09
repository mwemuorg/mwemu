use crate::emu;
use crate::winapi::helper;

pub fn CertEnumPhysicalStore(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CertEnumPhysicalStore arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = helper::handler_create("CertEnumPhysicalStore");
}
