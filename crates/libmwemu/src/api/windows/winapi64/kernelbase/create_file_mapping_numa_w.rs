use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingNumaW(emu: &mut emu::Emu) {
    let file_handle = emu.regs().rcx;
    let security = emu.regs().rdx;
    let protect = emu.regs().r8;
    let max_size_high = emu.regs().r9;
    let max_size_low = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let name_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
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
