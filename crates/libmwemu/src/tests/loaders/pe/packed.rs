use std::cell::Cell;
use std::error::Error;
use std::fs;
use std::rc::Rc;

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu64;
use crate::tests::helpers;
use crate::winapi::policy::ApiMode;
use crate::windows::constants::controls::WS_VISIBLE;

const SAMPLE: &str = "exe64win_enigma.bin";

#[test]
fn enigma_shows_window_and_polls_messages() -> Result<(), Box<dyn Error>> {
    const INSTRUCTION_LIMIT: u64 = 450_000_000;
    const REQUIRED_POLLS: usize = 3;

    helpers::setup();
    let mut emu = emu64();
    emu.cfg.console_enabled = false;
    emu.cfg.skip_unimplemented = true;
    emu.maps.set_banzai(true);
    emu.set_api_mode(ApiMode::SimulateSuccess);
    emu.set_seed(0);
    emu.cfg.winver = None;
    emu.set_maps_folder(&helpers::system_dlls64());
    let sample = sample!(SAMPLE);
    emu.load_code(&sample)?;
    let drive = helpers::file_system_root().join("c");
    fs::create_dir_all(&drive)?;
    fs::copy(sample, drive.join(&emu.cfg.exe_name))?;

    let shown_window = Rc::new(Cell::new(0));
    let polls = Rc::new(Cell::new(0));
    let window = Rc::clone(&shown_window);
    let count = Rc::clone(&polls);
    emu.hooks.on_winapi_call(move |emu, _, address| {
        match emu.export_indexes.resolve_address_module("user32", address) {
            Some("ShowWindow") => {
                let call = WinApi(Stdcall);
                if let Ok(hwnd) = call.arg(emu, 0)
                    && emu.guest_windows.get(hwnd).is_some_and(|w| w.parent == 0)
                    && hwnd != window.get()
                {
                    window.set(hwnd);
                    count.set(0);
                }
            }
            Some("PeekMessageW")
                if emu
                    .guest_windows
                    .get(window.get())
                    .is_some_and(|w| w.style & WS_VISIBLE != 0) =>
            {
                count.set(count.get() + 1);
                if count.get() == REQUIRED_POLLS {
                    emu.stop();
                }
            }
            _ => {}
        }
        true
    });

    let outcome = emu.run_to(INSTRUCTION_LIMIT);
    assert_eq!(
        polls.get(),
        REQUIRED_POLLS,
        "Enigma stopped at instruction {} (PC {:#x}, shown HWND {:#x}, run {outcome:?})",
        emu.pos,
        emu.pc(),
        shown_window.get()
    );
    outcome?;
    Ok(())
}

#[test]
fn enigma_reaches_startup_budget() {
    const STARTUP_INSTRUCTIONS: u64 = 5_000_000;

    helpers::setup();
    let sample = sample!(SAMPLE);
    let mut emu = emu64();
    emu.cfg.winver = None;
    emu.cfg.maps_folder = helpers::system_dlls64();
    emu.load_code(&sample).expect("load Enigma");
    emu.run_to(STARTUP_INSTRUCTIONS)
        .expect("run Enigma startup");
    assert!(emu.pos >= STARTUP_INSTRUCTIONS, "stopped at {}", emu.pos);
}
