use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptAcquireContext(emu: &mut emu::Emu) {
    let h_prov_ptr = emu.regs().rcx;
    let container_ptr = emu.regs().rdx;
    let provider_ptr = emu.regs().r8;
    let prov_type = emu.regs().r9 as u32;
    let flags = WinApiAbi::arg(emu, 4) as u32;

    let container = if container_ptr != 0 {
        emu.maps.read_string(container_ptr)
    } else {
        String::new()
    };
    let provider = if provider_ptr != 0 {
        emu.maps.read_string(provider_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "crypt32!CryptAcquireContext container: '{}' provider: '{}' type: {} flags: 0x{:x}",
        container,
        provider,
        prov_type,
        flags
    );

    let h_prov = helper::handler_create(&format!("cryptprov:{}", prov_type));
    if h_prov_ptr != 0 {
        emu.maps.write_qword(h_prov_ptr, h_prov);
    }
    emu.regs_mut().rax = 1; // TRUE
}
