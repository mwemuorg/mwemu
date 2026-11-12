use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let security = emu.regs().rdx;
    let protect = emu.regs().r8;
    let max_size_high = emu.regs().r9;
    let max_size_low = WinApiAbi::arg(emu, 5);
    let name_ptr = WinApiAbi::arg(emu, 6);
    let name = if name_ptr != 0 {
        emu.maps.read_wide_string(name_ptr)
    } else {
        String::from("(anon)")
    };
    let hndl = helper::handler_create(&format!("filemap_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateFileMappingNumaW file:0x{:x} '{}' => 0x{:x}",
        file_handle,
        name,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
