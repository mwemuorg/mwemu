use crate::emu;
use crate::winapi::helper;

pub fn api_DeviceIoControl(emu: &mut emu::Emu) {
    let h_device = emu.regs().rcx;
    let io_control_code = emu.regs().rdx;
    let in_buffer = emu.regs().r8;
    let in_buffer_size = emu.regs().r9;
    let rsp = emu.regs().rsp;
    let out_buffer = emu.maps.read_qword(rsp + 0x20).unwrap_or(0);
    let out_buffer_size = emu.maps.read_qword(rsp + 0x28).unwrap_or(0);
    let bytes_returned = emu.maps.read_qword(rsp + 0x30).unwrap_or(0);
    let _overlapped = emu.maps.read_qword(rsp + 0x38).unwrap_or(0);

    log_red!(
        emu,
        "kernel32!DeviceIoControl h:0x{:x} code:0x{:x} in:0x{:x}({}) out:0x{:x}({})",
        h_device,
        io_control_code,
        in_buffer,
        in_buffer_size,
        out_buffer,
        out_buffer_size
    );

    if !helper::handler_exist(h_device) {
        emu.set_last_error(6);
        emu.regs_mut().rax = 0;
        return;
    }

    if bytes_returned != 0 {
        emu.maps.write_dword(bytes_returned, 0);
    }
    emu.regs_mut().rax = 1;
}
