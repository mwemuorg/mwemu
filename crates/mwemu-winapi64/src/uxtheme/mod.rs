use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod get_theme_app_properties;
use get_theme_app_properties::GetThemeAppProperties;
mod is_app_themed;
use is_app_themed::IsAppThemed;
mod is_theme_active;
use is_theme_active::IsThemeActive;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "IsAppThemed" => IsAppThemed(emu),
        "IsThemeActive" => IsThemeActive(emu),
        "GetThemeAppProperties" => GetThemeAppProperties(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}
