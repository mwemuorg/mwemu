use crate::emu;
use crate::winapi::helper;

/*
HANDLE CreateEventA(
  [in, optional] LPSECURITY_ATTRIBUTES lpEventAttributes,
  [in]           BOOL                  bManualReset,
  [in]           BOOL                  bInitialState,
  [in, optional] LPCSTR                lpName
);

*/
pub fn CreateEventA(emu: &mut emu::Emu) {
    let attributes = emu.regs().rcx;
    let bManualReset = emu.regs().rdx;
    let bInitialState = emu.regs().r8;
    let name_ptr = emu.regs().r9;

    let mut name = String::new();
    if name_ptr > 0 {
        name = emu.maps.read_string(name_ptr);
    }

    log_red!(
        emu,
        "kernel32!CreateEventA attr: 0x{:x} manual_reset: {} init_state: {} name: {}",
        attributes,
        bManualReset,
        bInitialState,
        name
    );

    let uri = format!("event://{}", name);
    emu.regs_mut().rax = helper::handler_create(&uri);
}
