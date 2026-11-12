use crate::emu::Emu;
use crate::winapi::winapi64::ole32;

mod clipformat_user_free;
pub use clipformat_user_free::CLIPFORMAT_UserFree;

mod clipformat_user_free64;
pub use clipformat_user_free64::CLIPFORMAT_UserFree64;

mod clipformat_user_free_ext;
pub use clipformat_user_free_ext::CLIPFORMAT_UserFreeExt;

mod clipformat_user_marshal;
pub use clipformat_user_marshal::CLIPFORMAT_UserMarshal;

mod clipformat_user_marshal64;
pub use clipformat_user_marshal64::CLIPFORMAT_UserMarshal64;

mod clipformat_user_marshal_ext;
pub use clipformat_user_marshal_ext::CLIPFORMAT_UserMarshalExt;

mod clipformat_user_size;
pub use clipformat_user_size::CLIPFORMAT_UserSize;

mod clipformat_user_size64;
pub use clipformat_user_size64::CLIPFORMAT_UserSize64;

mod clipformat_user_size_ext;
pub use clipformat_user_size_ext::CLIPFORMAT_UserSizeExt;

mod clipformat_user_unmarshal;
pub use clipformat_user_unmarshal::CLIPFORMAT_UserUnmarshal;

mod clipformat_user_unmarshal64;
pub use clipformat_user_unmarshal64::CLIPFORMAT_UserUnmarshal64;

mod clipformat_user_unmarshal_ext;
pub use clipformat_user_unmarshal_ext::CLIPFORMAT_UserUnmarshalExt;

mod c_std_async_stub_buffer2_connect;
pub use c_std_async_stub_buffer2_connect::CStdAsyncStubBuffer2_Connect;

mod c_std_async_stub_buffer2_disconnect;
pub use c_std_async_stub_buffer2_disconnect::CStdAsyncStubBuffer2_Disconnect;

mod c_std_async_stub_buffer2_release;
pub use c_std_async_stub_buffer2_release::CStdAsyncStubBuffer2_Release;

mod c_std_async_stub_buffer_add_ref;
pub use c_std_async_stub_buffer_add_ref::CStdAsyncStubBuffer_AddRef;

mod c_std_async_stub_buffer_connect;
pub use c_std_async_stub_buffer_connect::CStdAsyncStubBuffer_Connect;

mod c_std_async_stub_buffer_disconnect;
pub use c_std_async_stub_buffer_disconnect::CStdAsyncStubBuffer_Disconnect;

mod c_std_async_stub_buffer_invoke;
pub use c_std_async_stub_buffer_invoke::CStdAsyncStubBuffer_Invoke;

mod c_std_async_stub_buffer_query_interface;
pub use c_std_async_stub_buffer_query_interface::CStdAsyncStubBuffer_QueryInterface;

mod c_std_async_stub_buffer_release;
pub use c_std_async_stub_buffer_release::CStdAsyncStubBuffer_Release;

mod c_std_stub_buffer2_connect;
pub use c_std_stub_buffer2_connect::CStdStubBuffer2_Connect;

mod c_std_stub_buffer2_count_refs;
pub use c_std_stub_buffer2_count_refs::CStdStubBuffer2_CountRefs;

mod c_std_stub_buffer2_disconnect;
pub use c_std_stub_buffer2_disconnect::CStdStubBuffer2_Disconnect;

mod c_std_stub_buffer2_query_interface;
pub use c_std_stub_buffer2_query_interface::CStdStubBuffer2_QueryInterface;

mod co_copy_proxy;
pub use co_copy_proxy::CoCopyProxy;

mod co_create_free_threaded_marshaler;
pub use co_create_free_threaded_marshaler::CoCreateFreeThreadedMarshaler;

mod co_decode_proxy;
pub use co_decode_proxy::CoDecodeProxy;

mod co_get_marshal_size_max;
pub use co_get_marshal_size_max::CoGetMarshalSizeMax;

mod co_get_standard_marshal;
pub use co_get_standard_marshal::CoGetStandardMarshal;

mod co_get_std_marshal_ex;
pub use co_get_std_marshal_ex::CoGetStdMarshalEx;

mod co_marshal_hresult;
pub use co_marshal_hresult::CoMarshalHresult;

mod co_marshal_inter_thread_interface_in_stream;
pub use co_marshal_inter_thread_interface_in_stream::CoMarshalInterThreadInterfaceInStream;

mod co_marshal_interface;
pub use co_marshal_interface::CoMarshalInterface;

mod co_query_proxy_blanket;
pub use co_query_proxy_blanket::CoQueryProxyBlanket;

mod co_register_channel_hook;
pub use co_register_channel_hook::CoRegisterChannelHook;

mod co_release_marshal_data;
pub use co_release_marshal_data::CoReleaseMarshalData;

mod co_set_proxy_blanket;
pub use co_set_proxy_blanket::CoSetProxyBlanket;

mod com_ps_ndr_dll_can_unload_now;
pub use com_ps_ndr_dll_can_unload_now::ComPs_NdrDllCanUnloadNow;

mod com_ps_ndr_dll_get_class_object;
pub use com_ps_ndr_dll_get_class_object::ComPs_NdrDllGetClassObject;

mod com_ps_ndr_dll_register_proxy;
pub use com_ps_ndr_dll_register_proxy::ComPs_NdrDllRegisterProxy;

mod com_ps_ndr_dll_unregister_proxy;
pub use com_ps_ndr_dll_unregister_proxy::ComPs_NdrDllUnregisterProxy;

mod dcom_channel_set_h_result;
pub use dcom_channel_set_h_result::DcomChannelSetHResult;

mod dll_debug_object_rpc_hook;
pub use dll_debug_object_rpc_hook::DllDebugObjectRPCHook;

mod haccel_user_free;
pub use haccel_user_free::HACCEL_UserFree;

mod haccel_user_free64;
pub use haccel_user_free64::HACCEL_UserFree64;

mod haccel_user_marshal;
pub use haccel_user_marshal::HACCEL_UserMarshal;

mod haccel_user_marshal64;
pub use haccel_user_marshal64::HACCEL_UserMarshal64;

mod haccel_user_size;
pub use haccel_user_size::HACCEL_UserSize;

mod haccel_user_size64;
pub use haccel_user_size64::HACCEL_UserSize64;

mod haccel_user_unmarshal;
pub use haccel_user_unmarshal::HACCEL_UserUnmarshal;

mod haccel_user_unmarshal64;
pub use haccel_user_unmarshal64::HACCEL_UserUnmarshal64;

mod hbitmap_user_free;
pub use hbitmap_user_free::HBITMAP_UserFree;

mod hbitmap_user_free64;
pub use hbitmap_user_free64::HBITMAP_UserFree64;

mod hbitmap_user_marshal;
pub use hbitmap_user_marshal::HBITMAP_UserMarshal;

mod hbitmap_user_marshal64;
pub use hbitmap_user_marshal64::HBITMAP_UserMarshal64;

mod hbitmap_user_size;
pub use hbitmap_user_size::HBITMAP_UserSize;

mod hbitmap_user_size64;
pub use hbitmap_user_size64::HBITMAP_UserSize64;

mod hbitmap_user_unmarshal;
pub use hbitmap_user_unmarshal::HBITMAP_UserUnmarshal;

mod hbitmap_user_unmarshal64;
pub use hbitmap_user_unmarshal64::HBITMAP_UserUnmarshal64;

mod hbrush_user_free;
pub use hbrush_user_free::HBRUSH_UserFree;

mod hbrush_user_free64;
pub use hbrush_user_free64::HBRUSH_UserFree64;

mod hbrush_user_marshal;
pub use hbrush_user_marshal::HBRUSH_UserMarshal;

mod hbrush_user_marshal64;
pub use hbrush_user_marshal64::HBRUSH_UserMarshal64;

mod hbrush_user_size;
pub use hbrush_user_size::HBRUSH_UserSize;

mod hbrush_user_size64;
pub use hbrush_user_size64::HBRUSH_UserSize64;

mod hbrush_user_unmarshal;
pub use hbrush_user_unmarshal::HBRUSH_UserUnmarshal;

mod hbrush_user_unmarshal64;
pub use hbrush_user_unmarshal64::HBRUSH_UserUnmarshal64;

mod hdc_user_free;
pub use hdc_user_free::HDC_UserFree;

mod hdc_user_free64;
pub use hdc_user_free64::HDC_UserFree64;

mod hdc_user_marshal;
pub use hdc_user_marshal::HDC_UserMarshal;

mod hdc_user_marshal64;
pub use hdc_user_marshal64::HDC_UserMarshal64;

mod hdc_user_size;
pub use hdc_user_size::HDC_UserSize;

mod hdc_user_size64;
pub use hdc_user_size64::HDC_UserSize64;

mod hdc_user_unmarshal;
pub use hdc_user_unmarshal::HDC_UserUnmarshal;

mod hdc_user_unmarshal64;
pub use hdc_user_unmarshal64::HDC_UserUnmarshal64;

mod henhmetafile_user_free;
pub use henhmetafile_user_free::HENHMETAFILE_UserFree;

mod henhmetafile_user_free64;
pub use henhmetafile_user_free64::HENHMETAFILE_UserFree64;

mod henhmetafile_user_marshal;
pub use henhmetafile_user_marshal::HENHMETAFILE_UserMarshal;

mod henhmetafile_user_marshal64;
pub use henhmetafile_user_marshal64::HENHMETAFILE_UserMarshal64;

mod henhmetafile_user_size;
pub use henhmetafile_user_size::HENHMETAFILE_UserSize;

mod henhmetafile_user_size64;
pub use henhmetafile_user_size64::HENHMETAFILE_UserSize64;

mod henhmetafile_user_unmarshal;
pub use henhmetafile_user_unmarshal::HENHMETAFILE_UserUnmarshal;

mod henhmetafile_user_unmarshal64;
pub use henhmetafile_user_unmarshal64::HENHMETAFILE_UserUnmarshal64;

mod hglobal_user_free;
pub use hglobal_user_free::HGLOBAL_UserFree;

mod hglobal_user_free64;
pub use hglobal_user_free64::HGLOBAL_UserFree64;

mod hglobal_user_marshal;
pub use hglobal_user_marshal::HGLOBAL_UserMarshal;

mod hglobal_user_marshal64;
pub use hglobal_user_marshal64::HGLOBAL_UserMarshal64;

mod hglobal_user_size;
pub use hglobal_user_size::HGLOBAL_UserSize;

mod hglobal_user_size64;
pub use hglobal_user_size64::HGLOBAL_UserSize64;

mod hglobal_user_unmarshal;
pub use hglobal_user_unmarshal::HGLOBAL_UserUnmarshal;

mod hglobal_user_unmarshal64;
pub use hglobal_user_unmarshal64::HGLOBAL_UserUnmarshal64;

mod hicon_user_free;
pub use hicon_user_free::HICON_UserFree;

mod hicon_user_free64;
pub use hicon_user_free64::HICON_UserFree64;

mod hicon_user_marshal;
pub use hicon_user_marshal::HICON_UserMarshal;

mod hicon_user_marshal64;
pub use hicon_user_marshal64::HICON_UserMarshal64;

mod hicon_user_size;
pub use hicon_user_size::HICON_UserSize;

mod hicon_user_size64;
pub use hicon_user_size64::HICON_UserSize64;

mod hicon_user_unmarshal;
pub use hicon_user_unmarshal::HICON_UserUnmarshal;

mod hicon_user_unmarshal64;
pub use hicon_user_unmarshal64::HICON_UserUnmarshal64;

mod hmenu_user_free;
pub use hmenu_user_free::HMENU_UserFree;

mod hmenu_user_free64;
pub use hmenu_user_free64::HMENU_UserFree64;

mod hmenu_user_marshal;
pub use hmenu_user_marshal::HMENU_UserMarshal;

mod hmenu_user_marshal64;
pub use hmenu_user_marshal64::HMENU_UserMarshal64;

mod hmenu_user_size;
pub use hmenu_user_size::HMENU_UserSize;

mod hmenu_user_size64;
pub use hmenu_user_size64::HMENU_UserSize64;

mod hmenu_user_unmarshal;
pub use hmenu_user_unmarshal::HMENU_UserUnmarshal;

mod hmenu_user_unmarshal64;
pub use hmenu_user_unmarshal64::HMENU_UserUnmarshal64;

mod hmetafilepict_user_free;
pub use hmetafilepict_user_free::HMETAFILEPICT_UserFree;

mod hmetafilepict_user_free64;
pub use hmetafilepict_user_free64::HMETAFILEPICT_UserFree64;

mod hmetafilepict_user_marshal;
pub use hmetafilepict_user_marshal::HMETAFILEPICT_UserMarshal;

mod hmetafilepict_user_marshal64;
pub use hmetafilepict_user_marshal64::HMETAFILEPICT_UserMarshal64;

mod hmetafilepict_user_size;
pub use hmetafilepict_user_size::HMETAFILEPICT_UserSize;

mod hmetafilepict_user_size64;
pub use hmetafilepict_user_size64::HMETAFILEPICT_UserSize64;

mod hmetafilepict_user_unmarshal;
pub use hmetafilepict_user_unmarshal::HMETAFILEPICT_UserUnmarshal;

mod hmetafilepict_user_unmarshal64;
pub use hmetafilepict_user_unmarshal64::HMETAFILEPICT_UserUnmarshal64;

mod hmetafile_user_free;
pub use hmetafile_user_free::HMETAFILE_UserFree;

mod hmetafile_user_free64;
pub use hmetafile_user_free64::HMETAFILE_UserFree64;

mod hmetafile_user_marshal;
pub use hmetafile_user_marshal::HMETAFILE_UserMarshal;

mod hmetafile_user_marshal64;
pub use hmetafile_user_marshal64::HMETAFILE_UserMarshal64;

mod hmetafile_user_size;
pub use hmetafile_user_size::HMETAFILE_UserSize;

mod hmetafile_user_size64;
pub use hmetafile_user_size64::HMETAFILE_UserSize64;

mod hmetafile_user_unmarshal;
pub use hmetafile_user_unmarshal::HMETAFILE_UserUnmarshal;

mod hmetafile_user_unmarshal64;
pub use hmetafile_user_unmarshal64::HMETAFILE_UserUnmarshal64;

mod hmonitor_user_free;
pub use hmonitor_user_free::HMONITOR_UserFree;

mod hmonitor_user_free64;
pub use hmonitor_user_free64::HMONITOR_UserFree64;

mod hmonitor_user_marshal;
pub use hmonitor_user_marshal::HMONITOR_UserMarshal;

mod hmonitor_user_marshal64;
pub use hmonitor_user_marshal64::HMONITOR_UserMarshal64;

mod hmonitor_user_size;
pub use hmonitor_user_size::HMONITOR_UserSize;

mod hmonitor_user_size64;
pub use hmonitor_user_size64::HMONITOR_UserSize64;

mod hmonitor_user_unmarshal;
pub use hmonitor_user_unmarshal::HMONITOR_UserUnmarshal;

mod hmonitor_user_unmarshal64;
pub use hmonitor_user_unmarshal64::HMONITOR_UserUnmarshal64;

mod hpalette_user_free;
pub use hpalette_user_free::HPALETTE_UserFree;

mod hpalette_user_free64;
pub use hpalette_user_free64::HPALETTE_UserFree64;

mod hpalette_user_free_ext;
pub use hpalette_user_free_ext::HPALETTE_UserFreeExt;

mod hpalette_user_marshal;
pub use hpalette_user_marshal::HPALETTE_UserMarshal;

mod hpalette_user_marshal64;
pub use hpalette_user_marshal64::HPALETTE_UserMarshal64;

mod hpalette_user_marshal_ext;
pub use hpalette_user_marshal_ext::HPALETTE_UserMarshalExt;

mod hpalette_user_size;
pub use hpalette_user_size::HPALETTE_UserSize;

mod hpalette_user_size64;
pub use hpalette_user_size64::HPALETTE_UserSize64;

mod hpalette_user_size_ext;
pub use hpalette_user_size_ext::HPALETTE_UserSizeExt;

mod hpalette_user_unmarshal;
pub use hpalette_user_unmarshal::HPALETTE_UserUnmarshal;

mod hpalette_user_unmarshal64;
pub use hpalette_user_unmarshal64::HPALETTE_UserUnmarshal64;

mod hpalette_user_unmarshal_ext;
pub use hpalette_user_unmarshal_ext::HPALETTE_UserUnmarshalExt;

mod hrgn_user_free;
pub use hrgn_user_free::HRGN_UserFree;

mod hrgn_user_marshal;
pub use hrgn_user_marshal::HRGN_UserMarshal;

mod hrgn_user_size;
pub use hrgn_user_size::HRGN_UserSize;

mod hrgn_user_unmarshal;
pub use hrgn_user_unmarshal::HRGN_UserUnmarshal;

mod hwnd_user_free;
pub use hwnd_user_free::HWND_UserFree;

mod hwnd_user_free64;
pub use hwnd_user_free64::HWND_UserFree64;

mod hwnd_user_free64_ext;
pub use hwnd_user_free64_ext::HWND_UserFree64Ext;

mod hwnd_user_free_ext;
pub use hwnd_user_free_ext::HWND_UserFreeExt;

mod hwnd_user_marshal;
pub use hwnd_user_marshal::HWND_UserMarshal;

mod hwnd_user_marshal64;
pub use hwnd_user_marshal64::HWND_UserMarshal64;

mod hwnd_user_marshal64_ext;
pub use hwnd_user_marshal64_ext::HWND_UserMarshal64Ext;

mod hwnd_user_marshal_ext;
pub use hwnd_user_marshal_ext::HWND_UserMarshalExt;

mod hwnd_user_size;
pub use hwnd_user_size::HWND_UserSize;

mod hwnd_user_size64;
pub use hwnd_user_size64::HWND_UserSize64;

mod hwnd_user_size64_ext;
pub use hwnd_user_size64_ext::HWND_UserSize64Ext;

mod hwnd_user_size_ext;
pub use hwnd_user_size_ext::HWND_UserSizeExt;

mod hwnd_user_unmarshal;
pub use hwnd_user_unmarshal::HWND_UserUnmarshal;

mod hwnd_user_unmarshal64;
pub use hwnd_user_unmarshal64::HWND_UserUnmarshal64;

mod hwnd_user_unmarshal64_ext;
pub use hwnd_user_unmarshal64_ext::HWND_UserUnmarshal64Ext;

mod hwnd_user_unmarshal_ext;
pub use hwnd_user_unmarshal_ext::HWND_UserUnmarshalExt;

mod ndr_ole_initialize_extension;
pub use ndr_ole_initialize_extension::NdrOleInitializeExtension;

mod ndr_proxy_forwarding_function10;
pub use ndr_proxy_forwarding_function10::NdrProxyForwardingFunction10;

mod ndr_proxy_forwarding_function11;
pub use ndr_proxy_forwarding_function11::NdrProxyForwardingFunction11;

mod ndr_proxy_forwarding_function12;
pub use ndr_proxy_forwarding_function12::NdrProxyForwardingFunction12;

mod ndr_proxy_forwarding_function13;
pub use ndr_proxy_forwarding_function13::NdrProxyForwardingFunction13;

mod ndr_proxy_forwarding_function14;
pub use ndr_proxy_forwarding_function14::NdrProxyForwardingFunction14;

mod ndr_proxy_forwarding_function15;
pub use ndr_proxy_forwarding_function15::NdrProxyForwardingFunction15;

mod ndr_proxy_forwarding_function16;
pub use ndr_proxy_forwarding_function16::NdrProxyForwardingFunction16;

mod ndr_proxy_forwarding_function17;
pub use ndr_proxy_forwarding_function17::NdrProxyForwardingFunction17;

mod ndr_proxy_forwarding_function18;
pub use ndr_proxy_forwarding_function18::NdrProxyForwardingFunction18;

mod ndr_proxy_forwarding_function19;
pub use ndr_proxy_forwarding_function19::NdrProxyForwardingFunction19;

mod ndr_proxy_forwarding_function20;
pub use ndr_proxy_forwarding_function20::NdrProxyForwardingFunction20;

mod ndr_proxy_forwarding_function21;
pub use ndr_proxy_forwarding_function21::NdrProxyForwardingFunction21;

mod ndr_proxy_forwarding_function22;
pub use ndr_proxy_forwarding_function22::NdrProxyForwardingFunction22;

mod ndr_proxy_forwarding_function23;
pub use ndr_proxy_forwarding_function23::NdrProxyForwardingFunction23;

mod ndr_proxy_forwarding_function24;
pub use ndr_proxy_forwarding_function24::NdrProxyForwardingFunction24;

mod ndr_proxy_forwarding_function25;
pub use ndr_proxy_forwarding_function25::NdrProxyForwardingFunction25;

mod ndr_proxy_forwarding_function26;
pub use ndr_proxy_forwarding_function26::NdrProxyForwardingFunction26;

mod ndr_proxy_forwarding_function27;
pub use ndr_proxy_forwarding_function27::NdrProxyForwardingFunction27;

mod ndr_proxy_forwarding_function28;
pub use ndr_proxy_forwarding_function28::NdrProxyForwardingFunction28;

mod ndr_proxy_forwarding_function29;
pub use ndr_proxy_forwarding_function29::NdrProxyForwardingFunction29;

mod ndr_proxy_forwarding_function3;
pub use ndr_proxy_forwarding_function3::NdrProxyForwardingFunction3;

mod ndr_proxy_forwarding_function30;
pub use ndr_proxy_forwarding_function30::NdrProxyForwardingFunction30;

mod ndr_proxy_forwarding_function31;
pub use ndr_proxy_forwarding_function31::NdrProxyForwardingFunction31;

mod ndr_proxy_forwarding_function32;
pub use ndr_proxy_forwarding_function32::NdrProxyForwardingFunction32;

mod ndr_proxy_forwarding_function4;
pub use ndr_proxy_forwarding_function4::NdrProxyForwardingFunction4;

mod ndr_proxy_forwarding_function5;
pub use ndr_proxy_forwarding_function5::NdrProxyForwardingFunction5;

mod ndr_proxy_forwarding_function6;
pub use ndr_proxy_forwarding_function6::NdrProxyForwardingFunction6;

mod ndr_proxy_forwarding_function7;
pub use ndr_proxy_forwarding_function7::NdrProxyForwardingFunction7;

mod ndr_proxy_forwarding_function8;
pub use ndr_proxy_forwarding_function8::NdrProxyForwardingFunction8;

mod ndr_proxy_forwarding_function9;
pub use ndr_proxy_forwarding_function9::NdrProxyForwardingFunction9;

mod object_stubless_client10;
pub use object_stubless_client10::ObjectStublessClient10;

mod object_stubless_client11;
pub use object_stubless_client11::ObjectStublessClient11;

mod object_stubless_client12;
pub use object_stubless_client12::ObjectStublessClient12;

mod object_stubless_client13;
pub use object_stubless_client13::ObjectStublessClient13;

mod object_stubless_client14;
pub use object_stubless_client14::ObjectStublessClient14;

mod object_stubless_client15;
pub use object_stubless_client15::ObjectStublessClient15;

mod object_stubless_client16;
pub use object_stubless_client16::ObjectStublessClient16;

mod object_stubless_client17;
pub use object_stubless_client17::ObjectStublessClient17;

mod object_stubless_client18;
pub use object_stubless_client18::ObjectStublessClient18;

mod object_stubless_client19;
pub use object_stubless_client19::ObjectStublessClient19;

mod object_stubless_client20;
pub use object_stubless_client20::ObjectStublessClient20;

mod object_stubless_client21;
pub use object_stubless_client21::ObjectStublessClient21;

mod object_stubless_client22;
pub use object_stubless_client22::ObjectStublessClient22;

mod object_stubless_client23;
pub use object_stubless_client23::ObjectStublessClient23;

mod object_stubless_client24;
pub use object_stubless_client24::ObjectStublessClient24;

mod object_stubless_client25;
pub use object_stubless_client25::ObjectStublessClient25;

mod object_stubless_client26;
pub use object_stubless_client26::ObjectStublessClient26;

mod object_stubless_client27;
pub use object_stubless_client27::ObjectStublessClient27;

mod object_stubless_client28;
pub use object_stubless_client28::ObjectStublessClient28;

mod object_stubless_client29;
pub use object_stubless_client29::ObjectStublessClient29;

mod object_stubless_client3;
pub use object_stubless_client3::ObjectStublessClient3;

mod object_stubless_client30;
pub use object_stubless_client30::ObjectStublessClient30;

mod object_stubless_client31;
pub use object_stubless_client31::ObjectStublessClient31;

mod object_stubless_client32;
pub use object_stubless_client32::ObjectStublessClient32;

mod object_stubless_client4;
pub use object_stubless_client4::ObjectStublessClient4;

mod object_stubless_client5;
pub use object_stubless_client5::ObjectStublessClient5;

mod object_stubless_client6;
pub use object_stubless_client6::ObjectStublessClient6;

mod object_stubless_client7;
pub use object_stubless_client7::ObjectStublessClient7;

mod object_stubless_client8;
pub use object_stubless_client8::ObjectStublessClient8;

mod object_stubless_client9;
pub use object_stubless_client9::ObjectStublessClient9;

mod snb_user_free;
pub use snb_user_free::SNB_UserFree;

mod snb_user_free64;
pub use snb_user_free64::SNB_UserFree64;

mod snb_user_marshal;
pub use snb_user_marshal::SNB_UserMarshal;

mod snb_user_marshal64;
pub use snb_user_marshal64::SNB_UserMarshal64;

mod snb_user_size;
pub use snb_user_size::SNB_UserSize;

mod snb_user_size64;
pub use snb_user_size64::SNB_UserSize64;

mod snb_user_unmarshal;
pub use snb_user_unmarshal::SNB_UserUnmarshal;

mod snb_user_unmarshal64;
pub use snb_user_unmarshal64::SNB_UserUnmarshal64;

mod stgmedium_user_free;
pub use stgmedium_user_free::STGMEDIUM_UserFree;

mod stgmedium_user_free64;
pub use stgmedium_user_free64::STGMEDIUM_UserFree64;

mod stgmedium_user_free_ext;
pub use stgmedium_user_free_ext::STGMEDIUM_UserFreeExt;

mod stgmedium_user_marshal;
pub use stgmedium_user_marshal::STGMEDIUM_UserMarshal;

mod stgmedium_user_marshal64;
pub use stgmedium_user_marshal64::STGMEDIUM_UserMarshal64;

mod stgmedium_user_marshal_ext;
pub use stgmedium_user_marshal_ext::STGMEDIUM_UserMarshalExt;

mod stgmedium_user_size;
pub use stgmedium_user_size::STGMEDIUM_UserSize;

mod stgmedium_user_size64;
pub use stgmedium_user_size64::STGMEDIUM_UserSize64;

mod stgmedium_user_size_ext;
pub use stgmedium_user_size_ext::STGMEDIUM_UserSizeExt;

mod stgmedium_user_unmarshal;
pub use stgmedium_user_unmarshal::STGMEDIUM_UserUnmarshal;

mod stgmedium_user_unmarshal64;
pub use stgmedium_user_unmarshal64::STGMEDIUM_UserUnmarshal64;

mod stgmedium_user_unmarshal_ext;
pub use stgmedium_user_unmarshal_ext::STGMEDIUM_UserUnmarshalExt;

mod wdtp_interface_pointer_user_free;
pub use wdtp_interface_pointer_user_free::WdtpInterfacePointer_UserFree;

mod wdtp_interface_pointer_user_free64;
pub use wdtp_interface_pointer_user_free64::WdtpInterfacePointer_UserFree64;

mod wdtp_interface_pointer_user_marshal;
pub use wdtp_interface_pointer_user_marshal::WdtpInterfacePointer_UserMarshal;

mod wdtp_interface_pointer_user_marshal64;
pub use wdtp_interface_pointer_user_marshal64::WdtpInterfacePointer_UserMarshal64;

mod wdtp_interface_pointer_user_size;
pub use wdtp_interface_pointer_user_size::WdtpInterfacePointer_UserSize;

mod wdtp_interface_pointer_user_size64;
pub use wdtp_interface_pointer_user_size64::WdtpInterfacePointer_UserSize64;

mod wdtp_interface_pointer_user_unmarshal;
pub use wdtp_interface_pointer_user_unmarshal::WdtpInterfacePointer_UserUnmarshal;

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
