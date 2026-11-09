use crate::emu;
use super::ERROR_BUFFER_OVERFLOW;

pub(super) fn GetAdaptersInfo(emu: &mut emu::Emu) {
    let adapter_info = emu.regs().rcx;
    let out_buf_len = emu.regs().rdx;

    log_red!(
        emu,
        "iphlpapi!GetAdaptersInfo pAdapterInfo:0x{:x} pOutBufLen:0x{:x}",
        adapter_info,
        out_buf_len
    );

    if out_buf_len != 0 {
        emu.maps.write_dword(out_buf_len, 640);
    }

    emu.regs_mut().rax = ERROR_BUFFER_OVERFLOW;
}
