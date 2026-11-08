use crate::emu;
use crate::windows::constants;

pub fn CallNamedPipeA(emu: &mut emu::Emu) {
    let lpNamedPipeName = emu.regs().rcx as u32;
    let _lpInBuffer = emu.regs().rdx as u32;
    let _nInBufferSize = emu.regs().r8 as u32;
    let _lpOutBuffer = emu.regs().r9 as u32;
    let _nOutBufferSize = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _lpBytesRead = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _nTimeOut = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    let lpNamedPipeName = emu.maps.read_string(lpNamedPipeName as u64);

    log_red!(emu, "kernel32!CallNamedPipeA {}", lpNamedPipeName);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
