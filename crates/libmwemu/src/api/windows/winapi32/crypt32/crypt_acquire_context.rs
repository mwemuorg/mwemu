use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptAcquireContext(emu: &mut emu::Emu) {
    let h_prov_ptr = WinApiAbi::arg(emu, 0);
    let container_ptr = WinApiAbi::arg(emu, 1);
    let provider_ptr = WinApiAbi::arg(emu, 2);
    let prov_type = WinApiAbi::arg(emu, 3) as u32;
    let flags = WinApiAbi::arg(emu, 4) as u32;
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
