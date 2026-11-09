use crate::emu;
use crate::winapi::helper;

pub fn CreateNamedPipeW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let arg8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    log_red!(
        emu,
        "kernelbase!CreateNamedPipeW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x} arg8:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7,
        arg8
    );
    let hndl = helper::handler_create("pipe://CreateNamedPipeW");
    emu.regs_mut().rax = hndl;
}
