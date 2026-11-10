use crate::emu;
use crate::winapi::helper;

pub fn GetFinalPathNameByHandleA(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let uri = helper::handler_get_uri(handle);
    let path = uri.strip_prefix("file://").unwrap_or("C:\\unknown");
    log_red!(
        emu,
        "kernelbase!GetFinalPathNameByHandleA h:0x{:x} => '{}'",
        handle,
        path
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf != 0 && buf_size > 0 {
        emu.maps.write_string(buf, path);
    }
    emu.regs_mut().rax = path.len() as u64;
}

pub fn GetFinalPathNameByHandleW(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let buf_size = emu.regs().r8;
    let uri = helper::handler_get_uri(handle);
    let path = uri.strip_prefix("file://").unwrap_or("C:\\unknown");
    log_red!(
        emu,
        "kernelbase!GetFinalPathNameByHandleW h:0x{:x} => '{}'",
        handle,
        path
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if buf != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf, path);
    }
    emu.regs_mut().rax = path.len() as u64;
}
