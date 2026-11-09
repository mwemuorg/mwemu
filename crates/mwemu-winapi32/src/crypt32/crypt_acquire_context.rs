use crate::emu;
use crate::winapi::helper;

pub fn CryptAcquireContext(emu: &mut emu::Emu) {
    let h_prov_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let container_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let provider_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let prov_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let container = if container_ptr != 0 {
        emu.maps.read_string(container_ptr)
    } else {
        String::new()
    };
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptAcquireContext container: '{}' type: {} flags: 0x{:x}",
        container,
        prov_type,
        flags
    );
    let h_prov = helper::handler_create(&format!("cryptprov:{}", prov_type));
    if h_prov_ptr != 0 {
        emu.maps.write_dword(h_prov_ptr, h_prov as u32);
    }
    emu.regs_mut().rax = 1; // TRUE
}
