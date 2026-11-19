use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CommandLineToArgvW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cmd_ptr = api_try!(emu, call.arg(emu, 0));
    let argc_ptr = api_try!(emu, call.arg(emu, 1));
    let cmd = emu.maps.read_wide_string(cmd_ptr);
    if argc_ptr != 0 {
        emu.maps.write_dword(argc_ptr, 1);
    }
    let argv_alloc = emu
        .maps
        .alloc(256)
        .expect("shell32!CommandLineToArgvW alloc");
    emu.maps.write_dword(argv_alloc, (argv_alloc + 4) as u32);
    emu.maps.write_wide_string(argv_alloc + 4, &cmd);
    log_red!(
        emu,
        "shell32!CommandLineToArgvW cmd:'{}' => 0x{:x}",
        cmd,
        argv_alloc
    );
    call.cleanup_args(emu, 2);
    call.set_return(emu, argv_alloc);
}
