use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::modules::native_module;
pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api,{
"LdrLoadDll" | "LdrLoadDll_gul"=>native_module(emu,false),
"LdrGetDllHandleEx"=>native_module(emu,true),
},handled(())=>true,fallback=>false)
}
