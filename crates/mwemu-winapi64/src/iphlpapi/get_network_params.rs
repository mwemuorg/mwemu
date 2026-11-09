use crate::emu;
use super::ERROR_BUFFER_OVERFLOW;

pub(super) fn GetNetworkParams(emu: &mut emu::Emu) {
    let fixed_info = emu.regs().rcx;
    let out_buf_len = emu.regs().rdx;

    log_red!(
        emu,
        "iphlpapi!GetNetworkParams pFixedInfo:0x{:x} pOutBufLen:0x{:x}",
        fixed_info,
        out_buf_len
    );

    if out_buf_len != 0 {
        emu.maps.write_dword(out_buf_len, 312);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}
