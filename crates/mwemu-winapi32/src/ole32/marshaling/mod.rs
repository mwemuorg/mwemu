use crate::emu::Emu;
use crate::winapi::winapi32::ole32;

#[path = "../clipformat_user_free.rs"]
mod clipformat_user_free;
pub use clipformat_user_free::CLIPFORMAT_UserFree;

#[path = "../clipformat_user_free64.rs"]
mod clipformat_user_free64;
pub use clipformat_user_free64::CLIPFORMAT_UserFree64;

#[path = "../clipformat_user_free_ext.rs"]
mod clipformat_user_free_ext;
pub use clipformat_user_free_ext::CLIPFORMAT_UserFreeExt;

#[path = "../clipformat_user_marshal.rs"]
mod clipformat_user_marshal;
pub use clipformat_user_marshal::CLIPFORMAT_UserMarshal;

#[path = "../clipformat_user_marshal64.rs"]
mod clipformat_user_marshal64;
pub use clipformat_user_marshal64::CLIPFORMAT_UserMarshal64;

#[path = "../clipformat_user_marshal_ext.rs"]
mod clipformat_user_marshal_ext;
pub use clipformat_user_marshal_ext::CLIPFORMAT_UserMarshalExt;

#[path = "../clipformat_user_size.rs"]
mod clipformat_user_size;
pub use clipformat_user_size::CLIPFORMAT_UserSize;

#[path = "../clipformat_user_size64.rs"]
mod clipformat_user_size64;
pub use clipformat_user_size64::CLIPFORMAT_UserSize64;

#[path = "../clipformat_user_size_ext.rs"]
mod clipformat_user_size_ext;
pub use clipformat_user_size_ext::CLIPFORMAT_UserSizeExt;

#[path = "../clipformat_user_unmarshal.rs"]
mod clipformat_user_unmarshal;
pub use clipformat_user_unmarshal::CLIPFORMAT_UserUnmarshal;

#[path = "../clipformat_user_unmarshal64.rs"]
mod clipformat_user_unmarshal64;
pub use clipformat_user_unmarshal64::CLIPFORMAT_UserUnmarshal64;

#[path = "../clipformat_user_unmarshal_ext.rs"]
mod clipformat_user_unmarshal_ext;
pub use clipformat_user_unmarshal_ext::CLIPFORMAT_UserUnmarshalExt;

#[path = "../c_std_async_stub_buffer2_connect.rs"]
mod c_std_async_stub_buffer2_connect;
pub use c_std_async_stub_buffer2_connect::CStdAsyncStubBuffer2_Connect;

#[path = "../c_std_async_stub_buffer2_disconnect.rs"]
mod c_std_async_stub_buffer2_disconnect;
pub use c_std_async_stub_buffer2_disconnect::CStdAsyncStubBuffer2_Disconnect;

#[path = "../c_std_async_stub_buffer2_release.rs"]
mod c_std_async_stub_buffer2_release;
pub use c_std_async_stub_buffer2_release::CStdAsyncStubBuffer2_Release;

#[path = "../c_std_async_stub_buffer_add_ref.rs"]
mod c_std_async_stub_buffer_add_ref;
pub use c_std_async_stub_buffer_add_ref::CStdAsyncStubBuffer_AddRef;

#[path = "../c_std_async_stub_buffer_connect.rs"]
mod c_std_async_stub_buffer_connect;
pub use c_std_async_stub_buffer_connect::CStdAsyncStubBuffer_Connect;

#[path = "../c_std_async_stub_buffer_disconnect.rs"]
mod c_std_async_stub_buffer_disconnect;
pub use c_std_async_stub_buffer_disconnect::CStdAsyncStubBuffer_Disconnect;

#[path = "../c_std_async_stub_buffer_invoke.rs"]
mod c_std_async_stub_buffer_invoke;
pub use c_std_async_stub_buffer_invoke::CStdAsyncStubBuffer_Invoke;

#[path = "../c_std_async_stub_buffer_query_interface.rs"]
mod c_std_async_stub_buffer_query_interface;
pub use c_std_async_stub_buffer_query_interface::CStdAsyncStubBuffer_QueryInterface;

#[path = "../c_std_async_stub_buffer_release.rs"]
mod c_std_async_stub_buffer_release;
pub use c_std_async_stub_buffer_release::CStdAsyncStubBuffer_Release;

#[path = "../c_std_stub_buffer2_connect.rs"]
mod c_std_stub_buffer2_connect;
pub use c_std_stub_buffer2_connect::CStdStubBuffer2_Connect;

#[path = "../c_std_stub_buffer2_count_refs.rs"]
mod c_std_stub_buffer2_count_refs;
pub use c_std_stub_buffer2_count_refs::CStdStubBuffer2_CountRefs;

#[path = "../c_std_stub_buffer2_disconnect.rs"]
mod c_std_stub_buffer2_disconnect;
pub use c_std_stub_buffer2_disconnect::CStdStubBuffer2_Disconnect;

#[path = "../c_std_stub_buffer2_query_interface.rs"]
mod c_std_stub_buffer2_query_interface;
pub use c_std_stub_buffer2_query_interface::CStdStubBuffer2_QueryInterface;

#[path = "../co_copy_proxy.rs"]
mod co_copy_proxy;
pub use co_copy_proxy::CoCopyProxy;

#[path = "../co_create_free_threaded_marshaler.rs"]
mod co_create_free_threaded_marshaler;
pub use co_create_free_threaded_marshaler::CoCreateFreeThreadedMarshaler;

#[path = "../co_decode_proxy.rs"]
mod co_decode_proxy;
pub use co_decode_proxy::CoDecodeProxy;

#[path = "../co_get_marshal_size_max.rs"]
mod co_get_marshal_size_max;
pub use co_get_marshal_size_max::CoGetMarshalSizeMax;

#[path = "../co_get_standard_marshal.rs"]
mod co_get_standard_marshal;
pub use co_get_standard_marshal::CoGetStandardMarshal;

#[path = "../co_get_std_marshal_ex.rs"]
mod co_get_std_marshal_ex;
pub use co_get_std_marshal_ex::CoGetStdMarshalEx;

#[path = "../co_marshal_hresult.rs"]
mod co_marshal_hresult;
pub use co_marshal_hresult::CoMarshalHresult;

#[path = "../co_marshal_inter_thread_interface_in_stream.rs"]
mod co_marshal_inter_thread_interface_in_stream;
pub use co_marshal_inter_thread_interface_in_stream::CoMarshalInterThreadInterfaceInStream;

#[path = "../co_marshal_interface.rs"]
mod co_marshal_interface;
pub use co_marshal_interface::CoMarshalInterface;

#[path = "../co_query_proxy_blanket.rs"]
mod co_query_proxy_blanket;
pub use co_query_proxy_blanket::CoQueryProxyBlanket;

#[path = "../co_register_channel_hook.rs"]
mod co_register_channel_hook;
pub use co_register_channel_hook::CoRegisterChannelHook;

#[path = "../co_release_marshal_data.rs"]
mod co_release_marshal_data;
pub use co_release_marshal_data::CoReleaseMarshalData;

#[path = "../co_set_proxy_blanket.rs"]
mod co_set_proxy_blanket;
pub use co_set_proxy_blanket::CoSetProxyBlanket;

#[path = "../com_ps_ndr_dll_can_unload_now.rs"]
mod com_ps_ndr_dll_can_unload_now;
pub use com_ps_ndr_dll_can_unload_now::ComPs_NdrDllCanUnloadNow;

#[path = "../com_ps_ndr_dll_get_class_object.rs"]
mod com_ps_ndr_dll_get_class_object;
pub use com_ps_ndr_dll_get_class_object::ComPs_NdrDllGetClassObject;

#[path = "../com_ps_ndr_dll_register_proxy.rs"]
mod com_ps_ndr_dll_register_proxy;
pub use com_ps_ndr_dll_register_proxy::ComPs_NdrDllRegisterProxy;

#[path = "../com_ps_ndr_dll_unregister_proxy.rs"]
mod com_ps_ndr_dll_unregister_proxy;
pub use com_ps_ndr_dll_unregister_proxy::ComPs_NdrDllUnregisterProxy;

#[path = "../dcom_channel_set_h_result.rs"]
mod dcom_channel_set_h_result;
pub use dcom_channel_set_h_result::DcomChannelSetHResult;

#[path = "../dll_debug_object_rpc_hook.rs"]
mod dll_debug_object_rpc_hook;
pub use dll_debug_object_rpc_hook::DllDebugObjectRPCHook;

#[path = "../haccel_user_free.rs"]
mod haccel_user_free;
pub use haccel_user_free::HACCEL_UserFree;

#[path = "../haccel_user_free64.rs"]
mod haccel_user_free64;
pub use haccel_user_free64::HACCEL_UserFree64;

#[path = "../haccel_user_marshal.rs"]
mod haccel_user_marshal;
pub use haccel_user_marshal::HACCEL_UserMarshal;

#[path = "../haccel_user_marshal64.rs"]
mod haccel_user_marshal64;
pub use haccel_user_marshal64::HACCEL_UserMarshal64;

#[path = "../haccel_user_size.rs"]
mod haccel_user_size;
pub use haccel_user_size::HACCEL_UserSize;

#[path = "../haccel_user_size64.rs"]
mod haccel_user_size64;
pub use haccel_user_size64::HACCEL_UserSize64;

#[path = "../haccel_user_unmarshal.rs"]
mod haccel_user_unmarshal;
pub use haccel_user_unmarshal::HACCEL_UserUnmarshal;

#[path = "../haccel_user_unmarshal64.rs"]
mod haccel_user_unmarshal64;
pub use haccel_user_unmarshal64::HACCEL_UserUnmarshal64;

#[path = "../hbitmap_user_free.rs"]
mod hbitmap_user_free;
pub use hbitmap_user_free::HBITMAP_UserFree;

#[path = "../hbitmap_user_free64.rs"]
mod hbitmap_user_free64;
pub use hbitmap_user_free64::HBITMAP_UserFree64;

#[path = "../hbitmap_user_marshal.rs"]
mod hbitmap_user_marshal;
pub use hbitmap_user_marshal::HBITMAP_UserMarshal;

#[path = "../hbitmap_user_marshal64.rs"]
mod hbitmap_user_marshal64;
pub use hbitmap_user_marshal64::HBITMAP_UserMarshal64;

#[path = "../hbitmap_user_size.rs"]
mod hbitmap_user_size;
pub use hbitmap_user_size::HBITMAP_UserSize;

#[path = "../hbitmap_user_size64.rs"]
mod hbitmap_user_size64;
pub use hbitmap_user_size64::HBITMAP_UserSize64;

#[path = "../hbitmap_user_unmarshal.rs"]
mod hbitmap_user_unmarshal;
pub use hbitmap_user_unmarshal::HBITMAP_UserUnmarshal;

#[path = "../hbitmap_user_unmarshal64.rs"]
mod hbitmap_user_unmarshal64;
pub use hbitmap_user_unmarshal64::HBITMAP_UserUnmarshal64;

#[path = "../hbrush_user_free.rs"]
mod hbrush_user_free;
pub use hbrush_user_free::HBRUSH_UserFree;

#[path = "../hbrush_user_free64.rs"]
mod hbrush_user_free64;
pub use hbrush_user_free64::HBRUSH_UserFree64;

#[path = "../hbrush_user_marshal.rs"]
mod hbrush_user_marshal;
pub use hbrush_user_marshal::HBRUSH_UserMarshal;

#[path = "../hbrush_user_marshal64.rs"]
mod hbrush_user_marshal64;
pub use hbrush_user_marshal64::HBRUSH_UserMarshal64;

#[path = "../hbrush_user_size.rs"]
mod hbrush_user_size;
pub use hbrush_user_size::HBRUSH_UserSize;

#[path = "../hbrush_user_size64.rs"]
mod hbrush_user_size64;
pub use hbrush_user_size64::HBRUSH_UserSize64;

#[path = "../hbrush_user_unmarshal.rs"]
mod hbrush_user_unmarshal;
pub use hbrush_user_unmarshal::HBRUSH_UserUnmarshal;

#[path = "../hbrush_user_unmarshal64.rs"]
mod hbrush_user_unmarshal64;
pub use hbrush_user_unmarshal64::HBRUSH_UserUnmarshal64;

#[path = "../hdc_user_free.rs"]
mod hdc_user_free;
pub use hdc_user_free::HDC_UserFree;

#[path = "../hdc_user_free64.rs"]
mod hdc_user_free64;
pub use hdc_user_free64::HDC_UserFree64;

#[path = "../hdc_user_marshal.rs"]
mod hdc_user_marshal;
pub use hdc_user_marshal::HDC_UserMarshal;

#[path = "../hdc_user_marshal64.rs"]
mod hdc_user_marshal64;
pub use hdc_user_marshal64::HDC_UserMarshal64;

#[path = "../hdc_user_size.rs"]
mod hdc_user_size;
pub use hdc_user_size::HDC_UserSize;

#[path = "../hdc_user_size64.rs"]
mod hdc_user_size64;
pub use hdc_user_size64::HDC_UserSize64;

#[path = "../hdc_user_unmarshal.rs"]
mod hdc_user_unmarshal;
pub use hdc_user_unmarshal::HDC_UserUnmarshal;

#[path = "../hdc_user_unmarshal64.rs"]
mod hdc_user_unmarshal64;
pub use hdc_user_unmarshal64::HDC_UserUnmarshal64;

#[path = "../henhmetafile_user_free.rs"]
mod henhmetafile_user_free;
pub use henhmetafile_user_free::HENHMETAFILE_UserFree;

#[path = "../henhmetafile_user_free64.rs"]
mod henhmetafile_user_free64;
pub use henhmetafile_user_free64::HENHMETAFILE_UserFree64;

#[path = "../henhmetafile_user_marshal.rs"]
mod henhmetafile_user_marshal;
pub use henhmetafile_user_marshal::HENHMETAFILE_UserMarshal;

#[path = "../henhmetafile_user_marshal64.rs"]
mod henhmetafile_user_marshal64;
pub use henhmetafile_user_marshal64::HENHMETAFILE_UserMarshal64;

#[path = "../henhmetafile_user_size.rs"]
mod henhmetafile_user_size;
pub use henhmetafile_user_size::HENHMETAFILE_UserSize;

#[path = "../henhmetafile_user_size64.rs"]
mod henhmetafile_user_size64;
pub use henhmetafile_user_size64::HENHMETAFILE_UserSize64;

#[path = "../henhmetafile_user_unmarshal.rs"]
mod henhmetafile_user_unmarshal;
pub use henhmetafile_user_unmarshal::HENHMETAFILE_UserUnmarshal;

#[path = "../henhmetafile_user_unmarshal64.rs"]
mod henhmetafile_user_unmarshal64;
pub use henhmetafile_user_unmarshal64::HENHMETAFILE_UserUnmarshal64;

#[path = "../hglobal_user_free.rs"]
mod hglobal_user_free;
pub use hglobal_user_free::HGLOBAL_UserFree;

#[path = "../hglobal_user_free64.rs"]
mod hglobal_user_free64;
pub use hglobal_user_free64::HGLOBAL_UserFree64;

#[path = "../hglobal_user_marshal.rs"]
mod hglobal_user_marshal;
pub use hglobal_user_marshal::HGLOBAL_UserMarshal;

#[path = "../hglobal_user_marshal64.rs"]
mod hglobal_user_marshal64;
pub use hglobal_user_marshal64::HGLOBAL_UserMarshal64;

#[path = "../hglobal_user_size.rs"]
mod hglobal_user_size;
pub use hglobal_user_size::HGLOBAL_UserSize;

#[path = "../hglobal_user_size64.rs"]
mod hglobal_user_size64;
pub use hglobal_user_size64::HGLOBAL_UserSize64;

#[path = "../hglobal_user_unmarshal.rs"]
mod hglobal_user_unmarshal;
pub use hglobal_user_unmarshal::HGLOBAL_UserUnmarshal;

#[path = "../hglobal_user_unmarshal64.rs"]
mod hglobal_user_unmarshal64;
pub use hglobal_user_unmarshal64::HGLOBAL_UserUnmarshal64;

#[path = "../hicon_user_free.rs"]
mod hicon_user_free;
pub use hicon_user_free::HICON_UserFree;

#[path = "../hicon_user_free64.rs"]
mod hicon_user_free64;
pub use hicon_user_free64::HICON_UserFree64;

#[path = "../hicon_user_marshal.rs"]
mod hicon_user_marshal;
pub use hicon_user_marshal::HICON_UserMarshal;

#[path = "../hicon_user_marshal64.rs"]
mod hicon_user_marshal64;
pub use hicon_user_marshal64::HICON_UserMarshal64;

#[path = "../hicon_user_size.rs"]
mod hicon_user_size;
pub use hicon_user_size::HICON_UserSize;

#[path = "../hicon_user_size64.rs"]
mod hicon_user_size64;
pub use hicon_user_size64::HICON_UserSize64;

#[path = "../hicon_user_unmarshal.rs"]
mod hicon_user_unmarshal;
pub use hicon_user_unmarshal::HICON_UserUnmarshal;

#[path = "../hicon_user_unmarshal64.rs"]
mod hicon_user_unmarshal64;
pub use hicon_user_unmarshal64::HICON_UserUnmarshal64;

#[path = "../hmenu_user_free.rs"]
mod hmenu_user_free;
pub use hmenu_user_free::HMENU_UserFree;

#[path = "../hmenu_user_free64.rs"]
mod hmenu_user_free64;
pub use hmenu_user_free64::HMENU_UserFree64;

#[path = "../hmenu_user_marshal.rs"]
mod hmenu_user_marshal;
pub use hmenu_user_marshal::HMENU_UserMarshal;

#[path = "../hmenu_user_marshal64.rs"]
mod hmenu_user_marshal64;
pub use hmenu_user_marshal64::HMENU_UserMarshal64;

#[path = "../hmenu_user_size.rs"]
mod hmenu_user_size;
pub use hmenu_user_size::HMENU_UserSize;

#[path = "../hmenu_user_size64.rs"]
mod hmenu_user_size64;
pub use hmenu_user_size64::HMENU_UserSize64;

#[path = "../hmenu_user_unmarshal.rs"]
mod hmenu_user_unmarshal;
pub use hmenu_user_unmarshal::HMENU_UserUnmarshal;

#[path = "../hmenu_user_unmarshal64.rs"]
mod hmenu_user_unmarshal64;
pub use hmenu_user_unmarshal64::HMENU_UserUnmarshal64;

#[path = "../hmetafilepict_user_free.rs"]
mod hmetafilepict_user_free;
pub use hmetafilepict_user_free::HMETAFILEPICT_UserFree;

#[path = "../hmetafilepict_user_free64.rs"]
mod hmetafilepict_user_free64;
pub use hmetafilepict_user_free64::HMETAFILEPICT_UserFree64;

#[path = "../hmetafilepict_user_marshal.rs"]
mod hmetafilepict_user_marshal;
pub use hmetafilepict_user_marshal::HMETAFILEPICT_UserMarshal;

#[path = "../hmetafilepict_user_marshal64.rs"]
mod hmetafilepict_user_marshal64;
pub use hmetafilepict_user_marshal64::HMETAFILEPICT_UserMarshal64;

#[path = "../hmetafilepict_user_size.rs"]
mod hmetafilepict_user_size;
pub use hmetafilepict_user_size::HMETAFILEPICT_UserSize;

#[path = "../hmetafilepict_user_size64.rs"]
mod hmetafilepict_user_size64;
pub use hmetafilepict_user_size64::HMETAFILEPICT_UserSize64;

#[path = "../hmetafilepict_user_unmarshal.rs"]
mod hmetafilepict_user_unmarshal;
pub use hmetafilepict_user_unmarshal::HMETAFILEPICT_UserUnmarshal;

#[path = "../hmetafilepict_user_unmarshal64.rs"]
mod hmetafilepict_user_unmarshal64;
pub use hmetafilepict_user_unmarshal64::HMETAFILEPICT_UserUnmarshal64;

#[path = "../hmetafile_user_free.rs"]
mod hmetafile_user_free;
pub use hmetafile_user_free::HMETAFILE_UserFree;

#[path = "../hmetafile_user_free64.rs"]
mod hmetafile_user_free64;
pub use hmetafile_user_free64::HMETAFILE_UserFree64;

#[path = "../hmetafile_user_marshal.rs"]
mod hmetafile_user_marshal;
pub use hmetafile_user_marshal::HMETAFILE_UserMarshal;

#[path = "../hmetafile_user_marshal64.rs"]
mod hmetafile_user_marshal64;
pub use hmetafile_user_marshal64::HMETAFILE_UserMarshal64;

#[path = "../hmetafile_user_size.rs"]
mod hmetafile_user_size;
pub use hmetafile_user_size::HMETAFILE_UserSize;

#[path = "../hmetafile_user_size64.rs"]
mod hmetafile_user_size64;
pub use hmetafile_user_size64::HMETAFILE_UserSize64;

#[path = "../hmetafile_user_unmarshal.rs"]
mod hmetafile_user_unmarshal;
pub use hmetafile_user_unmarshal::HMETAFILE_UserUnmarshal;

#[path = "../hmetafile_user_unmarshal64.rs"]
mod hmetafile_user_unmarshal64;
pub use hmetafile_user_unmarshal64::HMETAFILE_UserUnmarshal64;

#[path = "../hmonitor_user_free.rs"]
mod hmonitor_user_free;
pub use hmonitor_user_free::HMONITOR_UserFree;

#[path = "../hmonitor_user_free64.rs"]
mod hmonitor_user_free64;
pub use hmonitor_user_free64::HMONITOR_UserFree64;

#[path = "../hmonitor_user_marshal.rs"]
mod hmonitor_user_marshal;
pub use hmonitor_user_marshal::HMONITOR_UserMarshal;

#[path = "../hmonitor_user_marshal64.rs"]
mod hmonitor_user_marshal64;
pub use hmonitor_user_marshal64::HMONITOR_UserMarshal64;

#[path = "../hmonitor_user_size.rs"]
mod hmonitor_user_size;
pub use hmonitor_user_size::HMONITOR_UserSize;

#[path = "../hmonitor_user_size64.rs"]
mod hmonitor_user_size64;
pub use hmonitor_user_size64::HMONITOR_UserSize64;

#[path = "../hmonitor_user_unmarshal.rs"]
mod hmonitor_user_unmarshal;
pub use hmonitor_user_unmarshal::HMONITOR_UserUnmarshal;

#[path = "../hmonitor_user_unmarshal64.rs"]
mod hmonitor_user_unmarshal64;
pub use hmonitor_user_unmarshal64::HMONITOR_UserUnmarshal64;

#[path = "../hpalette_user_free.rs"]
mod hpalette_user_free;
pub use hpalette_user_free::HPALETTE_UserFree;

#[path = "../hpalette_user_free64.rs"]
mod hpalette_user_free64;
pub use hpalette_user_free64::HPALETTE_UserFree64;

#[path = "../hpalette_user_free_ext.rs"]
mod hpalette_user_free_ext;
pub use hpalette_user_free_ext::HPALETTE_UserFreeExt;

#[path = "../hpalette_user_marshal.rs"]
mod hpalette_user_marshal;
pub use hpalette_user_marshal::HPALETTE_UserMarshal;

#[path = "../hpalette_user_marshal64.rs"]
mod hpalette_user_marshal64;
pub use hpalette_user_marshal64::HPALETTE_UserMarshal64;

#[path = "../hpalette_user_marshal_ext.rs"]
mod hpalette_user_marshal_ext;
pub use hpalette_user_marshal_ext::HPALETTE_UserMarshalExt;

#[path = "../hpalette_user_size.rs"]
mod hpalette_user_size;
pub use hpalette_user_size::HPALETTE_UserSize;

#[path = "../hpalette_user_size64.rs"]
mod hpalette_user_size64;
pub use hpalette_user_size64::HPALETTE_UserSize64;

#[path = "../hpalette_user_size_ext.rs"]
mod hpalette_user_size_ext;
pub use hpalette_user_size_ext::HPALETTE_UserSizeExt;

#[path = "../hpalette_user_unmarshal.rs"]
mod hpalette_user_unmarshal;
pub use hpalette_user_unmarshal::HPALETTE_UserUnmarshal;

#[path = "../hpalette_user_unmarshal64.rs"]
mod hpalette_user_unmarshal64;
pub use hpalette_user_unmarshal64::HPALETTE_UserUnmarshal64;

#[path = "../hpalette_user_unmarshal_ext.rs"]
mod hpalette_user_unmarshal_ext;
pub use hpalette_user_unmarshal_ext::HPALETTE_UserUnmarshalExt;

#[path = "../hrgn_user_free.rs"]
mod hrgn_user_free;
pub use hrgn_user_free::HRGN_UserFree;

#[path = "../hrgn_user_marshal.rs"]
mod hrgn_user_marshal;
pub use hrgn_user_marshal::HRGN_UserMarshal;

#[path = "../hrgn_user_size.rs"]
mod hrgn_user_size;
pub use hrgn_user_size::HRGN_UserSize;

#[path = "../hrgn_user_unmarshal.rs"]
mod hrgn_user_unmarshal;
pub use hrgn_user_unmarshal::HRGN_UserUnmarshal;

#[path = "../hwnd_user_free.rs"]
mod hwnd_user_free;
pub use hwnd_user_free::HWND_UserFree;

#[path = "../hwnd_user_free64.rs"]
mod hwnd_user_free64;
pub use hwnd_user_free64::HWND_UserFree64;

#[path = "../hwnd_user_free64_ext.rs"]
mod hwnd_user_free64_ext;
pub use hwnd_user_free64_ext::HWND_UserFree64Ext;

#[path = "../hwnd_user_free_ext.rs"]
mod hwnd_user_free_ext;
pub use hwnd_user_free_ext::HWND_UserFreeExt;

#[path = "../hwnd_user_marshal.rs"]
mod hwnd_user_marshal;
pub use hwnd_user_marshal::HWND_UserMarshal;

#[path = "../hwnd_user_marshal64.rs"]
mod hwnd_user_marshal64;
pub use hwnd_user_marshal64::HWND_UserMarshal64;

#[path = "../hwnd_user_marshal64_ext.rs"]
mod hwnd_user_marshal64_ext;
pub use hwnd_user_marshal64_ext::HWND_UserMarshal64Ext;

#[path = "../hwnd_user_marshal_ext.rs"]
mod hwnd_user_marshal_ext;
pub use hwnd_user_marshal_ext::HWND_UserMarshalExt;

#[path = "../hwnd_user_size.rs"]
mod hwnd_user_size;
pub use hwnd_user_size::HWND_UserSize;

#[path = "../hwnd_user_size64.rs"]
mod hwnd_user_size64;
pub use hwnd_user_size64::HWND_UserSize64;

#[path = "../hwnd_user_size64_ext.rs"]
mod hwnd_user_size64_ext;
pub use hwnd_user_size64_ext::HWND_UserSize64Ext;

#[path = "../hwnd_user_size_ext.rs"]
mod hwnd_user_size_ext;
pub use hwnd_user_size_ext::HWND_UserSizeExt;

#[path = "../hwnd_user_unmarshal.rs"]
mod hwnd_user_unmarshal;
pub use hwnd_user_unmarshal::HWND_UserUnmarshal;

#[path = "../hwnd_user_unmarshal64.rs"]
mod hwnd_user_unmarshal64;
pub use hwnd_user_unmarshal64::HWND_UserUnmarshal64;

#[path = "../hwnd_user_unmarshal64_ext.rs"]
mod hwnd_user_unmarshal64_ext;
pub use hwnd_user_unmarshal64_ext::HWND_UserUnmarshal64Ext;

#[path = "../hwnd_user_unmarshal_ext.rs"]
mod hwnd_user_unmarshal_ext;
pub use hwnd_user_unmarshal_ext::HWND_UserUnmarshalExt;

#[path = "../ndr_ole_initialize_extension.rs"]
mod ndr_ole_initialize_extension;
pub use ndr_ole_initialize_extension::NdrOleInitializeExtension;

#[path = "../ndr_proxy_forwarding_function10.rs"]
mod ndr_proxy_forwarding_function10;
pub use ndr_proxy_forwarding_function10::NdrProxyForwardingFunction10;

#[path = "../ndr_proxy_forwarding_function11.rs"]
mod ndr_proxy_forwarding_function11;
pub use ndr_proxy_forwarding_function11::NdrProxyForwardingFunction11;

#[path = "../ndr_proxy_forwarding_function12.rs"]
mod ndr_proxy_forwarding_function12;
pub use ndr_proxy_forwarding_function12::NdrProxyForwardingFunction12;

#[path = "../ndr_proxy_forwarding_function13.rs"]
mod ndr_proxy_forwarding_function13;
pub use ndr_proxy_forwarding_function13::NdrProxyForwardingFunction13;

#[path = "../ndr_proxy_forwarding_function14.rs"]
mod ndr_proxy_forwarding_function14;
pub use ndr_proxy_forwarding_function14::NdrProxyForwardingFunction14;

#[path = "../ndr_proxy_forwarding_function15.rs"]
mod ndr_proxy_forwarding_function15;
pub use ndr_proxy_forwarding_function15::NdrProxyForwardingFunction15;

#[path = "../ndr_proxy_forwarding_function16.rs"]
mod ndr_proxy_forwarding_function16;
pub use ndr_proxy_forwarding_function16::NdrProxyForwardingFunction16;

#[path = "../ndr_proxy_forwarding_function17.rs"]
mod ndr_proxy_forwarding_function17;
pub use ndr_proxy_forwarding_function17::NdrProxyForwardingFunction17;

#[path = "../ndr_proxy_forwarding_function18.rs"]
mod ndr_proxy_forwarding_function18;
pub use ndr_proxy_forwarding_function18::NdrProxyForwardingFunction18;

#[path = "../ndr_proxy_forwarding_function19.rs"]
mod ndr_proxy_forwarding_function19;
pub use ndr_proxy_forwarding_function19::NdrProxyForwardingFunction19;

#[path = "../ndr_proxy_forwarding_function20.rs"]
mod ndr_proxy_forwarding_function20;
pub use ndr_proxy_forwarding_function20::NdrProxyForwardingFunction20;

#[path = "../ndr_proxy_forwarding_function21.rs"]
mod ndr_proxy_forwarding_function21;
pub use ndr_proxy_forwarding_function21::NdrProxyForwardingFunction21;

#[path = "../ndr_proxy_forwarding_function22.rs"]
mod ndr_proxy_forwarding_function22;
pub use ndr_proxy_forwarding_function22::NdrProxyForwardingFunction22;

#[path = "../ndr_proxy_forwarding_function23.rs"]
mod ndr_proxy_forwarding_function23;
pub use ndr_proxy_forwarding_function23::NdrProxyForwardingFunction23;

#[path = "../ndr_proxy_forwarding_function24.rs"]
mod ndr_proxy_forwarding_function24;
pub use ndr_proxy_forwarding_function24::NdrProxyForwardingFunction24;

#[path = "../ndr_proxy_forwarding_function25.rs"]
mod ndr_proxy_forwarding_function25;
pub use ndr_proxy_forwarding_function25::NdrProxyForwardingFunction25;

#[path = "../ndr_proxy_forwarding_function26.rs"]
mod ndr_proxy_forwarding_function26;
pub use ndr_proxy_forwarding_function26::NdrProxyForwardingFunction26;

#[path = "../ndr_proxy_forwarding_function27.rs"]
mod ndr_proxy_forwarding_function27;
pub use ndr_proxy_forwarding_function27::NdrProxyForwardingFunction27;

#[path = "../ndr_proxy_forwarding_function28.rs"]
mod ndr_proxy_forwarding_function28;
pub use ndr_proxy_forwarding_function28::NdrProxyForwardingFunction28;

#[path = "../ndr_proxy_forwarding_function29.rs"]
mod ndr_proxy_forwarding_function29;
pub use ndr_proxy_forwarding_function29::NdrProxyForwardingFunction29;

#[path = "../ndr_proxy_forwarding_function3.rs"]
mod ndr_proxy_forwarding_function3;
pub use ndr_proxy_forwarding_function3::NdrProxyForwardingFunction3;

#[path = "../ndr_proxy_forwarding_function30.rs"]
mod ndr_proxy_forwarding_function30;
pub use ndr_proxy_forwarding_function30::NdrProxyForwardingFunction30;

#[path = "../ndr_proxy_forwarding_function31.rs"]
mod ndr_proxy_forwarding_function31;
pub use ndr_proxy_forwarding_function31::NdrProxyForwardingFunction31;

#[path = "../ndr_proxy_forwarding_function32.rs"]
mod ndr_proxy_forwarding_function32;
pub use ndr_proxy_forwarding_function32::NdrProxyForwardingFunction32;

#[path = "../ndr_proxy_forwarding_function4.rs"]
mod ndr_proxy_forwarding_function4;
pub use ndr_proxy_forwarding_function4::NdrProxyForwardingFunction4;

#[path = "../ndr_proxy_forwarding_function5.rs"]
mod ndr_proxy_forwarding_function5;
pub use ndr_proxy_forwarding_function5::NdrProxyForwardingFunction5;

#[path = "../ndr_proxy_forwarding_function6.rs"]
mod ndr_proxy_forwarding_function6;
pub use ndr_proxy_forwarding_function6::NdrProxyForwardingFunction6;

#[path = "../ndr_proxy_forwarding_function7.rs"]
mod ndr_proxy_forwarding_function7;
pub use ndr_proxy_forwarding_function7::NdrProxyForwardingFunction7;

#[path = "../ndr_proxy_forwarding_function8.rs"]
mod ndr_proxy_forwarding_function8;
pub use ndr_proxy_forwarding_function8::NdrProxyForwardingFunction8;

#[path = "../ndr_proxy_forwarding_function9.rs"]
mod ndr_proxy_forwarding_function9;
pub use ndr_proxy_forwarding_function9::NdrProxyForwardingFunction9;

#[path = "../object_stubless_client10.rs"]
mod object_stubless_client10;
pub use object_stubless_client10::ObjectStublessClient10;

#[path = "../object_stubless_client11.rs"]
mod object_stubless_client11;
pub use object_stubless_client11::ObjectStublessClient11;

#[path = "../object_stubless_client12.rs"]
mod object_stubless_client12;
pub use object_stubless_client12::ObjectStublessClient12;

#[path = "../object_stubless_client13.rs"]
mod object_stubless_client13;
pub use object_stubless_client13::ObjectStublessClient13;

#[path = "../object_stubless_client14.rs"]
mod object_stubless_client14;
pub use object_stubless_client14::ObjectStublessClient14;

#[path = "../object_stubless_client15.rs"]
mod object_stubless_client15;
pub use object_stubless_client15::ObjectStublessClient15;

#[path = "../object_stubless_client16.rs"]
mod object_stubless_client16;
pub use object_stubless_client16::ObjectStublessClient16;

#[path = "../object_stubless_client17.rs"]
mod object_stubless_client17;
pub use object_stubless_client17::ObjectStublessClient17;

#[path = "../object_stubless_client18.rs"]
mod object_stubless_client18;
pub use object_stubless_client18::ObjectStublessClient18;

#[path = "../object_stubless_client19.rs"]
mod object_stubless_client19;
pub use object_stubless_client19::ObjectStublessClient19;

#[path = "../object_stubless_client20.rs"]
mod object_stubless_client20;
pub use object_stubless_client20::ObjectStublessClient20;

#[path = "../object_stubless_client21.rs"]
mod object_stubless_client21;
pub use object_stubless_client21::ObjectStublessClient21;

#[path = "../object_stubless_client22.rs"]
mod object_stubless_client22;
pub use object_stubless_client22::ObjectStublessClient22;

#[path = "../object_stubless_client23.rs"]
mod object_stubless_client23;
pub use object_stubless_client23::ObjectStublessClient23;

#[path = "../object_stubless_client24.rs"]
mod object_stubless_client24;
pub use object_stubless_client24::ObjectStublessClient24;

#[path = "../object_stubless_client25.rs"]
mod object_stubless_client25;
pub use object_stubless_client25::ObjectStublessClient25;

#[path = "../object_stubless_client26.rs"]
mod object_stubless_client26;
pub use object_stubless_client26::ObjectStublessClient26;

#[path = "../object_stubless_client27.rs"]
mod object_stubless_client27;
pub use object_stubless_client27::ObjectStublessClient27;

#[path = "../object_stubless_client28.rs"]
mod object_stubless_client28;
pub use object_stubless_client28::ObjectStublessClient28;

#[path = "../object_stubless_client29.rs"]
mod object_stubless_client29;
pub use object_stubless_client29::ObjectStublessClient29;

#[path = "../object_stubless_client3.rs"]
mod object_stubless_client3;
pub use object_stubless_client3::ObjectStublessClient3;

#[path = "../object_stubless_client30.rs"]
mod object_stubless_client30;
pub use object_stubless_client30::ObjectStublessClient30;

#[path = "../object_stubless_client31.rs"]
mod object_stubless_client31;
pub use object_stubless_client31::ObjectStublessClient31;

#[path = "../object_stubless_client32.rs"]
mod object_stubless_client32;
pub use object_stubless_client32::ObjectStublessClient32;

#[path = "../object_stubless_client4.rs"]
mod object_stubless_client4;
pub use object_stubless_client4::ObjectStublessClient4;

#[path = "../object_stubless_client5.rs"]
mod object_stubless_client5;
pub use object_stubless_client5::ObjectStublessClient5;

#[path = "../object_stubless_client6.rs"]
mod object_stubless_client6;
pub use object_stubless_client6::ObjectStublessClient6;

#[path = "../object_stubless_client7.rs"]
mod object_stubless_client7;
pub use object_stubless_client7::ObjectStublessClient7;

#[path = "../object_stubless_client8.rs"]
mod object_stubless_client8;
pub use object_stubless_client8::ObjectStublessClient8;

#[path = "../object_stubless_client9.rs"]
mod object_stubless_client9;
pub use object_stubless_client9::ObjectStublessClient9;

#[path = "../snb_user_free.rs"]
mod snb_user_free;
pub use snb_user_free::SNB_UserFree;

#[path = "../snb_user_free64.rs"]
mod snb_user_free64;
pub use snb_user_free64::SNB_UserFree64;

#[path = "../snb_user_marshal.rs"]
mod snb_user_marshal;
pub use snb_user_marshal::SNB_UserMarshal;

#[path = "../snb_user_marshal64.rs"]
mod snb_user_marshal64;
pub use snb_user_marshal64::SNB_UserMarshal64;

#[path = "../snb_user_size.rs"]
mod snb_user_size;
pub use snb_user_size::SNB_UserSize;

#[path = "../snb_user_size64.rs"]
mod snb_user_size64;
pub use snb_user_size64::SNB_UserSize64;

#[path = "../snb_user_unmarshal.rs"]
mod snb_user_unmarshal;
pub use snb_user_unmarshal::SNB_UserUnmarshal;

#[path = "../snb_user_unmarshal64.rs"]
mod snb_user_unmarshal64;
pub use snb_user_unmarshal64::SNB_UserUnmarshal64;

#[path = "../stgmedium_user_free.rs"]
mod stgmedium_user_free;
pub use stgmedium_user_free::STGMEDIUM_UserFree;

#[path = "../stgmedium_user_free64.rs"]
mod stgmedium_user_free64;
pub use stgmedium_user_free64::STGMEDIUM_UserFree64;

#[path = "../stgmedium_user_free_ext.rs"]
mod stgmedium_user_free_ext;
pub use stgmedium_user_free_ext::STGMEDIUM_UserFreeExt;

#[path = "../stgmedium_user_marshal.rs"]
mod stgmedium_user_marshal;
pub use stgmedium_user_marshal::STGMEDIUM_UserMarshal;

#[path = "../stgmedium_user_marshal64.rs"]
mod stgmedium_user_marshal64;
pub use stgmedium_user_marshal64::STGMEDIUM_UserMarshal64;

#[path = "../stgmedium_user_marshal_ext.rs"]
mod stgmedium_user_marshal_ext;
pub use stgmedium_user_marshal_ext::STGMEDIUM_UserMarshalExt;

#[path = "../stgmedium_user_size.rs"]
mod stgmedium_user_size;
pub use stgmedium_user_size::STGMEDIUM_UserSize;

#[path = "../stgmedium_user_size64.rs"]
mod stgmedium_user_size64;
pub use stgmedium_user_size64::STGMEDIUM_UserSize64;

#[path = "../stgmedium_user_size_ext.rs"]
mod stgmedium_user_size_ext;
pub use stgmedium_user_size_ext::STGMEDIUM_UserSizeExt;

#[path = "../stgmedium_user_unmarshal.rs"]
mod stgmedium_user_unmarshal;
pub use stgmedium_user_unmarshal::STGMEDIUM_UserUnmarshal;

#[path = "../stgmedium_user_unmarshal64.rs"]
mod stgmedium_user_unmarshal64;
pub use stgmedium_user_unmarshal64::STGMEDIUM_UserUnmarshal64;

#[path = "../stgmedium_user_unmarshal_ext.rs"]
mod stgmedium_user_unmarshal_ext;
pub use stgmedium_user_unmarshal_ext::STGMEDIUM_UserUnmarshalExt;

#[path = "../wdtp_interface_pointer_user_free.rs"]
mod wdtp_interface_pointer_user_free;
pub use wdtp_interface_pointer_user_free::WdtpInterfacePointer_UserFree;

#[path = "../wdtp_interface_pointer_user_free64.rs"]
mod wdtp_interface_pointer_user_free64;
pub use wdtp_interface_pointer_user_free64::WdtpInterfacePointer_UserFree64;

#[path = "../wdtp_interface_pointer_user_marshal.rs"]
mod wdtp_interface_pointer_user_marshal;
pub use wdtp_interface_pointer_user_marshal::WdtpInterfacePointer_UserMarshal;

#[path = "../wdtp_interface_pointer_user_marshal64.rs"]
mod wdtp_interface_pointer_user_marshal64;
pub use wdtp_interface_pointer_user_marshal64::WdtpInterfacePointer_UserMarshal64;

#[path = "../wdtp_interface_pointer_user_size.rs"]
mod wdtp_interface_pointer_user_size;
pub use wdtp_interface_pointer_user_size::WdtpInterfacePointer_UserSize;

#[path = "../wdtp_interface_pointer_user_size64.rs"]
mod wdtp_interface_pointer_user_size64;
pub use wdtp_interface_pointer_user_size64::WdtpInterfacePointer_UserSize64;

#[path = "../wdtp_interface_pointer_user_unmarshal.rs"]
mod wdtp_interface_pointer_user_unmarshal;
pub use wdtp_interface_pointer_user_unmarshal::WdtpInterfacePointer_UserUnmarshal;

#[path = "../wdtp_interface_pointer_user_unmarshal64.rs"]
mod wdtp_interface_pointer_user_unmarshal64;
pub use wdtp_interface_pointer_user_unmarshal64::WdtpInterfacePointer_UserUnmarshal64;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CLIPFORMAT_UserFree" => ole32::CLIPFORMAT_UserFree(emu),
        "CLIPFORMAT_UserFree64" => ole32::CLIPFORMAT_UserFree64(emu),
        "CLIPFORMAT_UserFreeExt" => ole32::CLIPFORMAT_UserFreeExt(emu),
        "CLIPFORMAT_UserMarshal" => ole32::CLIPFORMAT_UserMarshal(emu),
        "CLIPFORMAT_UserMarshal64" => ole32::CLIPFORMAT_UserMarshal64(emu),
        "CLIPFORMAT_UserMarshalExt" => ole32::CLIPFORMAT_UserMarshalExt(emu),
        "CLIPFORMAT_UserSize" => ole32::CLIPFORMAT_UserSize(emu),
        "CLIPFORMAT_UserSize64" => ole32::CLIPFORMAT_UserSize64(emu),
        "CLIPFORMAT_UserSizeExt" => ole32::CLIPFORMAT_UserSizeExt(emu),
        "CLIPFORMAT_UserUnmarshal" => ole32::CLIPFORMAT_UserUnmarshal(emu),
        "CLIPFORMAT_UserUnmarshal64" => ole32::CLIPFORMAT_UserUnmarshal64(emu),
        "CLIPFORMAT_UserUnmarshalExt" => ole32::CLIPFORMAT_UserUnmarshalExt(emu),
        "CStdAsyncStubBuffer2_Connect" => ole32::CStdAsyncStubBuffer2_Connect(emu),
        "CStdAsyncStubBuffer2_Disconnect" => ole32::CStdAsyncStubBuffer2_Disconnect(emu),
        "CStdAsyncStubBuffer2_Release" => ole32::CStdAsyncStubBuffer2_Release(emu),
        "CStdAsyncStubBuffer_AddRef" => ole32::CStdAsyncStubBuffer_AddRef(emu),
        "CStdAsyncStubBuffer_Connect" => ole32::CStdAsyncStubBuffer_Connect(emu),
        "CStdAsyncStubBuffer_Disconnect" => ole32::CStdAsyncStubBuffer_Disconnect(emu),
        "CStdAsyncStubBuffer_Invoke" => ole32::CStdAsyncStubBuffer_Invoke(emu),
        "CStdAsyncStubBuffer_QueryInterface" => ole32::CStdAsyncStubBuffer_QueryInterface(emu),
        "CStdAsyncStubBuffer_Release" => ole32::CStdAsyncStubBuffer_Release(emu),
        "CStdStubBuffer2_Connect" => ole32::CStdStubBuffer2_Connect(emu),
        "CStdStubBuffer2_CountRefs" => ole32::CStdStubBuffer2_CountRefs(emu),
        "CStdStubBuffer2_Disconnect" => ole32::CStdStubBuffer2_Disconnect(emu),
        "CStdStubBuffer2_QueryInterface" => ole32::CStdStubBuffer2_QueryInterface(emu),
        "CoCopyProxy" => ole32::CoCopyProxy(emu),
        "CoCreateFreeThreadedMarshaler" => ole32::CoCreateFreeThreadedMarshaler(emu),
        "CoDecodeProxy" => ole32::CoDecodeProxy(emu),
        "CoGetMarshalSizeMax" => ole32::CoGetMarshalSizeMax(emu),
        "CoGetStandardMarshal" => ole32::CoGetStandardMarshal(emu),
        "CoGetStdMarshalEx" => ole32::CoGetStdMarshalEx(emu),
        "CoMarshalHresult" => ole32::CoMarshalHresult(emu),
        "CoMarshalInterThreadInterfaceInStream" => {
            ole32::CoMarshalInterThreadInterfaceInStream(emu)
        }
        "CoMarshalInterface" => ole32::CoMarshalInterface(emu),
        "CoQueryProxyBlanket" => ole32::CoQueryProxyBlanket(emu),
        "CoRegisterChannelHook" => ole32::CoRegisterChannelHook(emu),
        "CoReleaseMarshalData" => ole32::CoReleaseMarshalData(emu),
        "CoSetProxyBlanket" => ole32::CoSetProxyBlanket(emu),
        "ComPs_NdrDllCanUnloadNow" => ole32::ComPs_NdrDllCanUnloadNow(emu),
        "ComPs_NdrDllGetClassObject" => ole32::ComPs_NdrDllGetClassObject(emu),
        "ComPs_NdrDllRegisterProxy" => ole32::ComPs_NdrDllRegisterProxy(emu),
        "ComPs_NdrDllUnregisterProxy" => ole32::ComPs_NdrDllUnregisterProxy(emu),
        "DcomChannelSetHResult" => ole32::DcomChannelSetHResult(emu),
        "DllDebugObjectRPCHook" => ole32::DllDebugObjectRPCHook(emu),
        "HACCEL_UserFree" => ole32::HACCEL_UserFree(emu),
        "HACCEL_UserFree64" => ole32::HACCEL_UserFree64(emu),
        "HACCEL_UserMarshal" => ole32::HACCEL_UserMarshal(emu),
        "HACCEL_UserMarshal64" => ole32::HACCEL_UserMarshal64(emu),
        "HACCEL_UserSize" => ole32::HACCEL_UserSize(emu),
        "HACCEL_UserSize64" => ole32::HACCEL_UserSize64(emu),
        "HACCEL_UserUnmarshal" => ole32::HACCEL_UserUnmarshal(emu),
        "HACCEL_UserUnmarshal64" => ole32::HACCEL_UserUnmarshal64(emu),
        "HBITMAP_UserFree" => ole32::HBITMAP_UserFree(emu),
        "HBITMAP_UserFree64" => ole32::HBITMAP_UserFree64(emu),
        "HBITMAP_UserMarshal" => ole32::HBITMAP_UserMarshal(emu),
        "HBITMAP_UserMarshal64" => ole32::HBITMAP_UserMarshal64(emu),
        "HBITMAP_UserSize" => ole32::HBITMAP_UserSize(emu),
        "HBITMAP_UserSize64" => ole32::HBITMAP_UserSize64(emu),
        "HBITMAP_UserUnmarshal" => ole32::HBITMAP_UserUnmarshal(emu),
        "HBITMAP_UserUnmarshal64" => ole32::HBITMAP_UserUnmarshal64(emu),
        "HBRUSH_UserFree" => ole32::HBRUSH_UserFree(emu),
        "HBRUSH_UserFree64" => ole32::HBRUSH_UserFree64(emu),
        "HBRUSH_UserMarshal" => ole32::HBRUSH_UserMarshal(emu),
        "HBRUSH_UserMarshal64" => ole32::HBRUSH_UserMarshal64(emu),
        "HBRUSH_UserSize" => ole32::HBRUSH_UserSize(emu),
        "HBRUSH_UserSize64" => ole32::HBRUSH_UserSize64(emu),
        "HBRUSH_UserUnmarshal" => ole32::HBRUSH_UserUnmarshal(emu),
        "HBRUSH_UserUnmarshal64" => ole32::HBRUSH_UserUnmarshal64(emu),
        "HDC_UserFree" => ole32::HDC_UserFree(emu),
        "HDC_UserFree64" => ole32::HDC_UserFree64(emu),
        "HDC_UserMarshal" => ole32::HDC_UserMarshal(emu),
        "HDC_UserMarshal64" => ole32::HDC_UserMarshal64(emu),
        "HDC_UserSize" => ole32::HDC_UserSize(emu),
        "HDC_UserSize64" => ole32::HDC_UserSize64(emu),
        "HDC_UserUnmarshal" => ole32::HDC_UserUnmarshal(emu),
        "HDC_UserUnmarshal64" => ole32::HDC_UserUnmarshal64(emu),
        "HENHMETAFILE_UserFree" => ole32::HENHMETAFILE_UserFree(emu),
        "HENHMETAFILE_UserFree64" => ole32::HENHMETAFILE_UserFree64(emu),
        "HENHMETAFILE_UserMarshal" => ole32::HENHMETAFILE_UserMarshal(emu),
        "HENHMETAFILE_UserMarshal64" => ole32::HENHMETAFILE_UserMarshal64(emu),
        "HENHMETAFILE_UserSize" => ole32::HENHMETAFILE_UserSize(emu),
        "HENHMETAFILE_UserSize64" => ole32::HENHMETAFILE_UserSize64(emu),
        "HENHMETAFILE_UserUnmarshal" => ole32::HENHMETAFILE_UserUnmarshal(emu),
        "HENHMETAFILE_UserUnmarshal64" => ole32::HENHMETAFILE_UserUnmarshal64(emu),
        "HGLOBAL_UserFree" => ole32::HGLOBAL_UserFree(emu),
        "HGLOBAL_UserFree64" => ole32::HGLOBAL_UserFree64(emu),
        "HGLOBAL_UserMarshal" => ole32::HGLOBAL_UserMarshal(emu),
        "HGLOBAL_UserMarshal64" => ole32::HGLOBAL_UserMarshal64(emu),
        "HGLOBAL_UserSize" => ole32::HGLOBAL_UserSize(emu),
        "HGLOBAL_UserSize64" => ole32::HGLOBAL_UserSize64(emu),
        "HGLOBAL_UserUnmarshal" => ole32::HGLOBAL_UserUnmarshal(emu),
        "HGLOBAL_UserUnmarshal64" => ole32::HGLOBAL_UserUnmarshal64(emu),
        "HICON_UserFree" => ole32::HICON_UserFree(emu),
        "HICON_UserFree64" => ole32::HICON_UserFree64(emu),
        "HICON_UserMarshal" => ole32::HICON_UserMarshal(emu),
        "HICON_UserMarshal64" => ole32::HICON_UserMarshal64(emu),
        "HICON_UserSize" => ole32::HICON_UserSize(emu),
        "HICON_UserSize64" => ole32::HICON_UserSize64(emu),
        "HICON_UserUnmarshal" => ole32::HICON_UserUnmarshal(emu),
        "HICON_UserUnmarshal64" => ole32::HICON_UserUnmarshal64(emu),
        "HMENU_UserFree" => ole32::HMENU_UserFree(emu),
        "HMENU_UserFree64" => ole32::HMENU_UserFree64(emu),
        "HMENU_UserMarshal" => ole32::HMENU_UserMarshal(emu),
        "HMENU_UserMarshal64" => ole32::HMENU_UserMarshal64(emu),
        "HMENU_UserSize" => ole32::HMENU_UserSize(emu),
        "HMENU_UserSize64" => ole32::HMENU_UserSize64(emu),
        "HMENU_UserUnmarshal" => ole32::HMENU_UserUnmarshal(emu),
        "HMENU_UserUnmarshal64" => ole32::HMENU_UserUnmarshal64(emu),
        "HMETAFILEPICT_UserFree" => ole32::HMETAFILEPICT_UserFree(emu),
        "HMETAFILEPICT_UserFree64" => ole32::HMETAFILEPICT_UserFree64(emu),
        "HMETAFILEPICT_UserMarshal" => ole32::HMETAFILEPICT_UserMarshal(emu),
        "HMETAFILEPICT_UserMarshal64" => ole32::HMETAFILEPICT_UserMarshal64(emu),
        "HMETAFILEPICT_UserSize" => ole32::HMETAFILEPICT_UserSize(emu),
        "HMETAFILEPICT_UserSize64" => ole32::HMETAFILEPICT_UserSize64(emu),
        "HMETAFILEPICT_UserUnmarshal" => ole32::HMETAFILEPICT_UserUnmarshal(emu),
        "HMETAFILEPICT_UserUnmarshal64" => ole32::HMETAFILEPICT_UserUnmarshal64(emu),
        "HMETAFILE_UserFree" => ole32::HMETAFILE_UserFree(emu),
        "HMETAFILE_UserFree64" => ole32::HMETAFILE_UserFree64(emu),
        "HMETAFILE_UserMarshal" => ole32::HMETAFILE_UserMarshal(emu),
        "HMETAFILE_UserMarshal64" => ole32::HMETAFILE_UserMarshal64(emu),
        "HMETAFILE_UserSize" => ole32::HMETAFILE_UserSize(emu),
        "HMETAFILE_UserSize64" => ole32::HMETAFILE_UserSize64(emu),
        "HMETAFILE_UserUnmarshal" => ole32::HMETAFILE_UserUnmarshal(emu),
        "HMETAFILE_UserUnmarshal64" => ole32::HMETAFILE_UserUnmarshal64(emu),
        "HMONITOR_UserFree" => ole32::HMONITOR_UserFree(emu),
        "HMONITOR_UserFree64" => ole32::HMONITOR_UserFree64(emu),
        "HMONITOR_UserMarshal" => ole32::HMONITOR_UserMarshal(emu),
        "HMONITOR_UserMarshal64" => ole32::HMONITOR_UserMarshal64(emu),
        "HMONITOR_UserSize" => ole32::HMONITOR_UserSize(emu),
        "HMONITOR_UserSize64" => ole32::HMONITOR_UserSize64(emu),
        "HMONITOR_UserUnmarshal" => ole32::HMONITOR_UserUnmarshal(emu),
        "HMONITOR_UserUnmarshal64" => ole32::HMONITOR_UserUnmarshal64(emu),
        "HPALETTE_UserFree" => ole32::HPALETTE_UserFree(emu),
        "HPALETTE_UserFree64" => ole32::HPALETTE_UserFree64(emu),
        "HPALETTE_UserFreeExt" => ole32::HPALETTE_UserFreeExt(emu),
        "HPALETTE_UserMarshal" => ole32::HPALETTE_UserMarshal(emu),
        "HPALETTE_UserMarshal64" => ole32::HPALETTE_UserMarshal64(emu),
        "HPALETTE_UserMarshalExt" => ole32::HPALETTE_UserMarshalExt(emu),
        "HPALETTE_UserSize" => ole32::HPALETTE_UserSize(emu),
        "HPALETTE_UserSize64" => ole32::HPALETTE_UserSize64(emu),
        "HPALETTE_UserSizeExt" => ole32::HPALETTE_UserSizeExt(emu),
        "HPALETTE_UserUnmarshal" => ole32::HPALETTE_UserUnmarshal(emu),
        "HPALETTE_UserUnmarshal64" => ole32::HPALETTE_UserUnmarshal64(emu),
        "HPALETTE_UserUnmarshalExt" => ole32::HPALETTE_UserUnmarshalExt(emu),
        "HRGN_UserFree" => ole32::HRGN_UserFree(emu),
        "HRGN_UserMarshal" => ole32::HRGN_UserMarshal(emu),
        "HRGN_UserSize" => ole32::HRGN_UserSize(emu),
        "HRGN_UserUnmarshal" => ole32::HRGN_UserUnmarshal(emu),
        "HWND_UserFree" => ole32::HWND_UserFree(emu),
        "HWND_UserFree64" => ole32::HWND_UserFree64(emu),
        "HWND_UserFree64Ext" => ole32::HWND_UserFree64Ext(emu),
        "HWND_UserFreeExt" => ole32::HWND_UserFreeExt(emu),
        "HWND_UserMarshal" => ole32::HWND_UserMarshal(emu),
        "HWND_UserMarshal64" => ole32::HWND_UserMarshal64(emu),
        "HWND_UserMarshal64Ext" => ole32::HWND_UserMarshal64Ext(emu),
        "HWND_UserMarshalExt" => ole32::HWND_UserMarshalExt(emu),
        "HWND_UserSize" => ole32::HWND_UserSize(emu),
        "HWND_UserSize64" => ole32::HWND_UserSize64(emu),
        "HWND_UserSize64Ext" => ole32::HWND_UserSize64Ext(emu),
        "HWND_UserSizeExt" => ole32::HWND_UserSizeExt(emu),
        "HWND_UserUnmarshal" => ole32::HWND_UserUnmarshal(emu),
        "HWND_UserUnmarshal64" => ole32::HWND_UserUnmarshal64(emu),
        "HWND_UserUnmarshal64Ext" => ole32::HWND_UserUnmarshal64Ext(emu),
        "HWND_UserUnmarshalExt" => ole32::HWND_UserUnmarshalExt(emu),
        "NdrOleInitializeExtension" => ole32::NdrOleInitializeExtension(emu),
        "NdrProxyForwardingFunction10" => ole32::NdrProxyForwardingFunction10(emu),
        "NdrProxyForwardingFunction11" => ole32::NdrProxyForwardingFunction11(emu),
        "NdrProxyForwardingFunction12" => ole32::NdrProxyForwardingFunction12(emu),
        "NdrProxyForwardingFunction13" => ole32::NdrProxyForwardingFunction13(emu),
        "NdrProxyForwardingFunction14" => ole32::NdrProxyForwardingFunction14(emu),
        "NdrProxyForwardingFunction15" => ole32::NdrProxyForwardingFunction15(emu),
        "NdrProxyForwardingFunction16" => ole32::NdrProxyForwardingFunction16(emu),
        "NdrProxyForwardingFunction17" => ole32::NdrProxyForwardingFunction17(emu),
        "NdrProxyForwardingFunction18" => ole32::NdrProxyForwardingFunction18(emu),
        "NdrProxyForwardingFunction19" => ole32::NdrProxyForwardingFunction19(emu),
        "NdrProxyForwardingFunction20" => ole32::NdrProxyForwardingFunction20(emu),
        "NdrProxyForwardingFunction21" => ole32::NdrProxyForwardingFunction21(emu),
        "NdrProxyForwardingFunction22" => ole32::NdrProxyForwardingFunction22(emu),
        "NdrProxyForwardingFunction23" => ole32::NdrProxyForwardingFunction23(emu),
        "NdrProxyForwardingFunction24" => ole32::NdrProxyForwardingFunction24(emu),
        "NdrProxyForwardingFunction25" => ole32::NdrProxyForwardingFunction25(emu),
        "NdrProxyForwardingFunction26" => ole32::NdrProxyForwardingFunction26(emu),
        "NdrProxyForwardingFunction27" => ole32::NdrProxyForwardingFunction27(emu),
        "NdrProxyForwardingFunction28" => ole32::NdrProxyForwardingFunction28(emu),
        "NdrProxyForwardingFunction29" => ole32::NdrProxyForwardingFunction29(emu),
        "NdrProxyForwardingFunction3" => ole32::NdrProxyForwardingFunction3(emu),
        "NdrProxyForwardingFunction30" => ole32::NdrProxyForwardingFunction30(emu),
        "NdrProxyForwardingFunction31" => ole32::NdrProxyForwardingFunction31(emu),
        "NdrProxyForwardingFunction32" => ole32::NdrProxyForwardingFunction32(emu),
        "NdrProxyForwardingFunction4" => ole32::NdrProxyForwardingFunction4(emu),
        "NdrProxyForwardingFunction5" => ole32::NdrProxyForwardingFunction5(emu),
        "NdrProxyForwardingFunction6" => ole32::NdrProxyForwardingFunction6(emu),
        "NdrProxyForwardingFunction7" => ole32::NdrProxyForwardingFunction7(emu),
        "NdrProxyForwardingFunction8" => ole32::NdrProxyForwardingFunction8(emu),
        "NdrProxyForwardingFunction9" => ole32::NdrProxyForwardingFunction9(emu),
        "ObjectStublessClient10" => ole32::ObjectStublessClient10(emu),
        "ObjectStublessClient11" => ole32::ObjectStublessClient11(emu),
        "ObjectStublessClient12" => ole32::ObjectStublessClient12(emu),
        "ObjectStublessClient13" => ole32::ObjectStublessClient13(emu),
        "ObjectStublessClient14" => ole32::ObjectStublessClient14(emu),
        "ObjectStublessClient15" => ole32::ObjectStublessClient15(emu),
        "ObjectStublessClient16" => ole32::ObjectStublessClient16(emu),
        "ObjectStublessClient17" => ole32::ObjectStublessClient17(emu),
        "ObjectStublessClient18" => ole32::ObjectStublessClient18(emu),
        "ObjectStublessClient19" => ole32::ObjectStublessClient19(emu),
        "ObjectStublessClient20" => ole32::ObjectStublessClient20(emu),
        "ObjectStublessClient21" => ole32::ObjectStublessClient21(emu),
        "ObjectStublessClient22" => ole32::ObjectStublessClient22(emu),
        "ObjectStublessClient23" => ole32::ObjectStublessClient23(emu),
        "ObjectStublessClient24" => ole32::ObjectStublessClient24(emu),
        "ObjectStublessClient25" => ole32::ObjectStublessClient25(emu),
        "ObjectStublessClient26" => ole32::ObjectStublessClient26(emu),
        "ObjectStublessClient27" => ole32::ObjectStublessClient27(emu),
        "ObjectStublessClient28" => ole32::ObjectStublessClient28(emu),
        "ObjectStublessClient29" => ole32::ObjectStublessClient29(emu),
        "ObjectStublessClient3" => ole32::ObjectStublessClient3(emu),
        "ObjectStublessClient30" => ole32::ObjectStublessClient30(emu),
        "ObjectStublessClient31" => ole32::ObjectStublessClient31(emu),
        "ObjectStublessClient32" => ole32::ObjectStublessClient32(emu),
        "ObjectStublessClient4" => ole32::ObjectStublessClient4(emu),
        "ObjectStublessClient5" => ole32::ObjectStublessClient5(emu),
        "ObjectStublessClient6" => ole32::ObjectStublessClient6(emu),
        "ObjectStublessClient7" => ole32::ObjectStublessClient7(emu),
        "ObjectStublessClient8" => ole32::ObjectStublessClient8(emu),
        "ObjectStublessClient9" => ole32::ObjectStublessClient9(emu),
        "SNB_UserFree" => ole32::SNB_UserFree(emu),
        "SNB_UserFree64" => ole32::SNB_UserFree64(emu),
        "SNB_UserMarshal" => ole32::SNB_UserMarshal(emu),
        "SNB_UserMarshal64" => ole32::SNB_UserMarshal64(emu),
        "SNB_UserSize" => ole32::SNB_UserSize(emu),
        "SNB_UserSize64" => ole32::SNB_UserSize64(emu),
        "SNB_UserUnmarshal" => ole32::SNB_UserUnmarshal(emu),
        "SNB_UserUnmarshal64" => ole32::SNB_UserUnmarshal64(emu),
        "STGMEDIUM_UserFree" => ole32::STGMEDIUM_UserFree(emu),
        "STGMEDIUM_UserFree64" => ole32::STGMEDIUM_UserFree64(emu),
        "STGMEDIUM_UserFreeExt" => ole32::STGMEDIUM_UserFreeExt(emu),
        "STGMEDIUM_UserMarshal" => ole32::STGMEDIUM_UserMarshal(emu),
        "STGMEDIUM_UserMarshal64" => ole32::STGMEDIUM_UserMarshal64(emu),
        "STGMEDIUM_UserMarshalExt" => ole32::STGMEDIUM_UserMarshalExt(emu),
        "STGMEDIUM_UserSize" => ole32::STGMEDIUM_UserSize(emu),
        "STGMEDIUM_UserSize64" => ole32::STGMEDIUM_UserSize64(emu),
        "STGMEDIUM_UserSizeExt" => ole32::STGMEDIUM_UserSizeExt(emu),
        "STGMEDIUM_UserUnmarshal" => ole32::STGMEDIUM_UserUnmarshal(emu),
        "STGMEDIUM_UserUnmarshal64" => ole32::STGMEDIUM_UserUnmarshal64(emu),
        "STGMEDIUM_UserUnmarshalExt" => ole32::STGMEDIUM_UserUnmarshalExt(emu),
        "WdtpInterfacePointer_UserFree" => ole32::WdtpInterfacePointer_UserFree(emu),
        "WdtpInterfacePointer_UserFree64" => ole32::WdtpInterfacePointer_UserFree64(emu),
        "WdtpInterfacePointer_UserMarshal" => ole32::WdtpInterfacePointer_UserMarshal(emu),
        "WdtpInterfacePointer_UserMarshal64" => ole32::WdtpInterfacePointer_UserMarshal64(emu),
        "WdtpInterfacePointer_UserSize" => ole32::WdtpInterfacePointer_UserSize(emu),
        "WdtpInterfacePointer_UserSize64" => ole32::WdtpInterfacePointer_UserSize64(emu),
        "WdtpInterfacePointer_UserUnmarshal" => ole32::WdtpInterfacePointer_UserUnmarshal(emu),
        "WdtpInterfacePointer_UserUnmarshal64" => ole32::WdtpInterfacePointer_UserUnmarshal64(emu),
        _ => return false,
    }
    true
}
