use crate::emu;
use crate::serialization;
use crate::winapi::winapi64;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "MessageBoxA" => MessageBoxA(emu),
        "GetDesktopWindow" => GetDesktopWindow(emu),
        "GetSystemMetrics" => GetSystemMetrics(emu),
        "SystemParametersInfoA" => SystemParametersInfoA(emu),
        "LoadIconA" => LoadIconA(emu),
        "LoadCursorA" => LoadCursorA(emu),
        "RegisterClassA" => RegisterClassA(emu),
        "RegisterClassW" => RegisterClassW(emu),
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
        }
    }
    String::new()
}

fn MessageBoxA(emu: &mut emu::Emu) {
    let titleptr = emu.regs().rcx;
    let msgptr = emu.regs().rdx;
    let msg = emu.maps.read_string(msgptr);
    let title = emu.maps.read_string(titleptr);

    log_red!(emu, "user32!MessageBoxA {} {}", title, msg);

    emu.regs_mut().rax = 0;
}

fn GetDesktopWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetDesktopWindow");
    //emu.regs_mut().rax = 0x11223344; // current window handle
    emu.regs_mut().rax = 0; // no windows handler is more stealthy
}

/*
int GetSystemMetrics(
  [in] int nIndex
);
*/
fn GetSystemMetrics(emu: &mut emu::Emu) {
    let nindex = emu.regs().rcx as usize;
    log_red!(emu, "user32!GetSystemMetrics nindex: {}", nindex);
    // TODO: do something
    emu.regs_mut().rax = 0;
}

/*
BOOL SystemParametersInfoA(
  [in]      UINT  uiAction,
  [in]      UINT  uiParam,
  [in, out] PVOID pvParam,
  [in]      UINT  fWinIni
);
*/
fn SystemParametersInfoA(emu: &mut emu::Emu) {
    let ui_action = emu.regs().rcx;
    let ui_param = emu.regs().rdx;
    let pv_param = emu.regs().r8;
    let f_win_ini = emu.regs().r9;
    log_red!(
        emu,
        "** {} user32!SystemParametersInfoA {} {} {} {}",
        emu.pos,
        ui_action,
        ui_param,
        pv_param,
        f_win_ini
    );
    // TODO: write pvParam
    emu.regs_mut().rax = 1;
}

/*
HICON LoadIconA(
[in, optional] HINSTANCE hInstance,
[in]           LPCSTR    lpIconName
);
*/
fn LoadIconA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let lpiconname = emu.regs().rdx;
    log_red!(
        emu,
        "** {} user32!LoadIconA {} {}",
        emu.pos,
        hinstance,
        lpiconname
    );
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
HCURSOR LoadCursorA(
  [in, optional] HINSTANCE hInstance,
  [in]           LPCSTR    lpCursorName
);
*/
fn LoadCursorA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let lpcursorname = emu.regs().rdx;
    log_red!(
        emu,
        "** {} user32!LoadCursorA {} {}",
        emu.pos,
        hinstance,
        lpcursorname
    );
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
ATOM RegisterClassA(
  [in] const WNDCLASSA *lpWndClass
);
*/
fn RegisterClassA(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "** {} user32!RegisterClassA {}", emu.pos, lpwndclass);
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
ATOM RegisterClassW(
  [in] const WNDCLASSW *lpWndClass
);
*/
fn RegisterClassW(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "** {} user32!RegisterClassW {}", emu.pos, lpwndclass);
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
HDC GetDC(
  [in] HWND hWnd
);
*/
fn GetDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "** {} user32!GetDC {}", emu.pos, hwnd);
    // TODO: do something / do not return null
    emu.regs_mut().rax = 0;
}

/*
int ReleaseDC(
  [in] HWND hWnd,
  [in] HDC  hDC
);
*/
fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let hdc = emu.regs().rdx;
    log_red!(emu, "** {} user32!ReleaseDC {} {}", emu.pos, hwnd, hdc);
    // TODO: do something
    emu.regs_mut().rax = 1;
}
