use crate::emu::Emu;
use crate::winapi::winapi64::ole32;

mod create_class_moniker;
pub use create_class_moniker::CreateClassMoniker;

mod get_running_object_table;
pub use get_running_object_table::GetRunningObjectTable;

mod bind_moniker;
pub use bind_moniker::BindMoniker;

mod check_init_dde;
pub use check_init_dde::CheckInitDde;

mod clean_rot_for_apartment;
pub use clean_rot_for_apartment::CleanROTForApartment;

mod clipboard_process_uninitialize;
pub use clipboard_process_uninitialize::ClipboardProcessUninitialize;

mod create_anti_moniker;
pub use create_anti_moniker::CreateAntiMoniker;

mod create_bind_ctx;
pub use create_bind_ctx::CreateBindCtx;

mod create_data_advise_holder;
pub use create_data_advise_holder::CreateDataAdviseHolder;

mod create_data_cache;
pub use create_data_cache::CreateDataCache;

mod create_file_moniker;
pub use create_file_moniker::CreateFileMoniker;

mod create_generic_composite;
pub use create_generic_composite::CreateGenericComposite;

mod create_item_moniker;
pub use create_item_moniker::CreateItemMoniker;

mod create_objref_moniker;
pub use create_objref_moniker::CreateObjrefMoniker;

mod create_ole_advise_holder;
pub use create_ole_advise_holder::CreateOleAdviseHolder;

mod create_pointer_moniker;
pub use create_pointer_moniker::CreatePointerMoniker;

mod dde_bind_to_object;
pub use dde_bind_to_object::DdeBindToObject;

mod destroy_running_object_table;
pub use destroy_running_object_table::DestroyRunningObjectTable;

mod do_drag_drop;
pub use do_drag_drop::DoDragDrop;

mod drag_drop_set_fdt;
pub use drag_drop_set_fdt::DragDropSetFDT;

mod get_object_from_rot_by_path;
pub use get_object_from_rot_by_path::GetObjectFromRotByPath;

mod hk_ole_register_object;
pub use hk_ole_register_object::HkOleRegisterObject;

mod is_accelerator;
pub use is_accelerator::IsAccelerator;

mod moniker_common_prefix_with;
pub use moniker_common_prefix_with::MonikerCommonPrefixWith;

mod moniker_load_type_lib;
pub use moniker_load_type_lib::MonikerLoadTypeLib;

mod moniker_relative_path_to;
pub use moniker_relative_path_to::MonikerRelativePathTo;

mod ole32_dll_get_class_object;
pub use ole32_dll_get_class_object::Ole32DllGetClassObject;

mod ole_build_version;
pub use ole_build_version::OleBuildVersion;

mod ole_create;
pub use ole_create::OleCreate;

mod ole_create_default_handler;
pub use ole_create_default_handler::OleCreateDefaultHandler;

mod ole_create_embedding_helper;
pub use ole_create_embedding_helper::OleCreateEmbeddingHelper;

mod ole_create_ex;
pub use ole_create_ex::OleCreateEx;

mod ole_create_font_indirect_ext;
pub use ole_create_font_indirect_ext::OleCreateFontIndirectExt;

mod ole_create_from_data;
pub use ole_create_from_data::OleCreateFromData;

mod ole_create_from_data_ex;
pub use ole_create_from_data_ex::OleCreateFromDataEx;

mod ole_create_from_file;
pub use ole_create_from_file::OleCreateFromFile;

mod ole_create_from_file_ex;
pub use ole_create_from_file_ex::OleCreateFromFileEx;

mod ole_create_link;
pub use ole_create_link::OleCreateLink;

mod ole_create_link_ex;
pub use ole_create_link_ex::OleCreateLinkEx;

mod ole_create_link_from_data;
pub use ole_create_link_from_data::OleCreateLinkFromData;

mod ole_create_link_from_data_ex;
pub use ole_create_link_from_data_ex::OleCreateLinkFromDataEx;

mod ole_create_link_to_file;
pub use ole_create_link_to_file::OleCreateLinkToFile;

mod ole_create_link_to_file_ex;
pub use ole_create_link_to_file_ex::OleCreateLinkToFileEx;

mod ole_create_menu_descriptor;
pub use ole_create_menu_descriptor::OleCreateMenuDescriptor;

mod ole_create_picture_indirect_ext;
pub use ole_create_picture_indirect_ext::OleCreatePictureIndirectExt;

mod ole_create_property_frame_indirect_ext;
pub use ole_create_property_frame_indirect_ext::OleCreatePropertyFrameIndirectExt;

mod ole_create_static_from_data;
pub use ole_create_static_from_data::OleCreateStaticFromData;

mod ole_destroy_menu_descriptor;
pub use ole_destroy_menu_descriptor::OleDestroyMenuDescriptor;

mod ole_do_auto_convert;
pub use ole_do_auto_convert::OleDoAutoConvert;

mod ole_draw;
pub use ole_draw::OleDraw;

mod ole_duplicate_data;
pub use ole_duplicate_data::OleDuplicateData;

mod ole_flush_clipboard;
pub use ole_flush_clipboard::OleFlushClipboard;

mod ole_get_auto_convert;
pub use ole_get_auto_convert::OleGetAutoConvert;

mod ole_get_clipboard;
pub use ole_get_clipboard::OleGetClipboard;

mod ole_get_clipboard_with_enterprise_info;
pub use ole_get_clipboard_with_enterprise_info::OleGetClipboardWithEnterpriseInfo;

mod ole_get_icon_of_class;
pub use ole_get_icon_of_class::OleGetIconOfClass;

mod ole_get_icon_of_file;
pub use ole_get_icon_of_file::OleGetIconOfFile;

mod ole_get_package_clipboard_owner;
pub use ole_get_package_clipboard_owner::OleGetPackageClipboardOwner;

mod ole_icon_to_cursor_ext;
pub use ole_icon_to_cursor_ext::OleIconToCursorExt;

mod ole_initialize_wow;
pub use ole_initialize_wow::OleInitializeWOW;

mod ole_is_current_clipboard;
pub use ole_is_current_clipboard::OleIsCurrentClipboard;

mod ole_is_running;
pub use ole_is_running::OleIsRunning;

mod ole_load;
pub use ole_load::OleLoad;

mod ole_load_picture_ext;
pub use ole_load_picture_ext::OleLoadPictureExt;

mod ole_load_picture_file_ext;
pub use ole_load_picture_file_ext::OleLoadPictureFileExt;

mod ole_load_picture_path_ext;
pub use ole_load_picture_path_ext::OleLoadPicturePathExt;

mod ole_lock_running;
pub use ole_lock_running::OleLockRunning;

mod ole_metafile_pict_from_icon_and_label;
pub use ole_metafile_pict_from_icon_and_label::OleMetafilePictFromIconAndLabel;

mod ole_note_object_visible;
pub use ole_note_object_visible::OleNoteObjectVisible;

mod ole_query_create_from_data;
pub use ole_query_create_from_data::OleQueryCreateFromData;

mod ole_query_link_from_data;
pub use ole_query_link_from_data::OleQueryLinkFromData;

mod ole_reg_enum_format_etc;
pub use ole_reg_enum_format_etc::OleRegEnumFormatEtc;

mod ole_reg_enum_verbs;
pub use ole_reg_enum_verbs::OleRegEnumVerbs;

mod ole_reg_get_misc_status;
pub use ole_reg_get_misc_status::OleRegGetMiscStatus;

mod ole_reg_get_user_type;
pub use ole_reg_get_user_type::OleRegGetUserType;

mod ole_release_enum_verb_cache;
pub use ole_release_enum_verb_cache::OleReleaseEnumVerbCache;

mod ole_run;
pub use ole_run::OleRun;

mod ole_save;
pub use ole_save::OleSave;

mod ole_save_picture_file_ext;
pub use ole_save_picture_file_ext::OleSavePictureFileExt;

mod ole_set_auto_convert;
pub use ole_set_auto_convert::OleSetAutoConvert;

mod ole_set_clipboard;
pub use ole_set_clipboard::OleSetClipboard;

mod ole_set_contained_object;
pub use ole_set_contained_object::OleSetContainedObject;

mod ole_set_menu_descriptor;
pub use ole_set_menu_descriptor::OleSetMenuDescriptor;

mod ole_translate_accelerator;
pub use ole_translate_accelerator::OleTranslateAccelerator;

mod ole_translate_color_ext;
pub use ole_translate_color_ext::OleTranslateColorExt;

mod ole_uninitialize;
pub use ole_uninitialize::OleUninitialize;

mod register_drag_drop;
pub use register_drag_drop::RegisterDragDrop;

mod revoke_drag_drop;
pub use revoke_drag_drop::RevokeDragDrop;

mod ut_convert_dvtd16to_dvtd32;
pub use ut_convert_dvtd16to_dvtd32::UtConvertDvtd16toDvtd32;

mod ut_convert_dvtd32to_dvtd16;
pub use ut_convert_dvtd32to_dvtd16::UtConvertDvtd32toDvtd16;

mod ut_get_dvtd16_info;
pub use ut_get_dvtd16_info::UtGetDvtd16Info;

mod ut_get_dvtd32_info;
pub use ut_get_dvtd32_info::UtGetDvtd32Info;

mod ole_initialize;
pub use ole_initialize::OleInitialize;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CreateClassMoniker" => ole32::CreateClassMoniker(emu),
        "GetRunningObjectTable" => ole32::GetRunningObjectTable(emu),
        "BindMoniker" => ole32::BindMoniker(emu),
        "CheckInitDde" => ole32::CheckInitDde(emu),
        "CleanROTForApartment" => ole32::CleanROTForApartment(emu),
        "ClipboardProcessUninitialize" => ole32::ClipboardProcessUninitialize(emu),
        "CreateAntiMoniker" => ole32::CreateAntiMoniker(emu),
        "CreateBindCtx" => ole32::CreateBindCtx(emu),
        "CreateDataAdviseHolder" => ole32::CreateDataAdviseHolder(emu),
        "CreateDataCache" => ole32::CreateDataCache(emu),
        "CreateFileMoniker" => ole32::CreateFileMoniker(emu),
        "CreateGenericComposite" => ole32::CreateGenericComposite(emu),
        "CreateItemMoniker" => ole32::CreateItemMoniker(emu),
        "CreateObjrefMoniker" => ole32::CreateObjrefMoniker(emu),
        "CreateOleAdviseHolder" => ole32::CreateOleAdviseHolder(emu),
        "CreatePointerMoniker" => ole32::CreatePointerMoniker(emu),
        "DdeBindToObject" => ole32::DdeBindToObject(emu),
        "DestroyRunningObjectTable" => ole32::DestroyRunningObjectTable(emu),
        "DoDragDrop" => ole32::DoDragDrop(emu),
        "DragDropSetFDT" => ole32::DragDropSetFDT(emu),
        "GetObjectFromRotByPath" => ole32::GetObjectFromRotByPath(emu),
        "HkOleRegisterObject" => ole32::HkOleRegisterObject(emu),
        "IsAccelerator" => ole32::IsAccelerator(emu),
        "MonikerCommonPrefixWith" => ole32::MonikerCommonPrefixWith(emu),
        "MonikerLoadTypeLib" => ole32::MonikerLoadTypeLib(emu),
        "MonikerRelativePathTo" => ole32::MonikerRelativePathTo(emu),
        "Ole32DllGetClassObject" => ole32::Ole32DllGetClassObject(emu),
        "OleCreate" => ole32::OleCreate(emu),
        "OleCreateDefaultHandler" => ole32::OleCreateDefaultHandler(emu),
        "OleCreateEmbeddingHelper" => ole32::OleCreateEmbeddingHelper(emu),
        "OleCreateEx" => ole32::OleCreateEx(emu),
        "OleCreateFontIndirectExt" => ole32::OleCreateFontIndirectExt(emu),
        "OleCreateFromData" => ole32::OleCreateFromData(emu),
        "OleCreateFromDataEx" => ole32::OleCreateFromDataEx(emu),
        "OleCreateFromFile" => ole32::OleCreateFromFile(emu),
        "OleCreateFromFileEx" => ole32::OleCreateFromFileEx(emu),
        "OleCreateLink" => ole32::OleCreateLink(emu),
        "OleCreateLinkEx" => ole32::OleCreateLinkEx(emu),
        "OleCreateLinkFromData" => ole32::OleCreateLinkFromData(emu),
        "OleCreateLinkFromDataEx" => ole32::OleCreateLinkFromDataEx(emu),
        "OleCreateLinkToFile" => ole32::OleCreateLinkToFile(emu),
        "OleCreateLinkToFileEx" => ole32::OleCreateLinkToFileEx(emu),
        "OleCreateMenuDescriptor" => ole32::OleCreateMenuDescriptor(emu),
        "OleCreatePictureIndirectExt" => ole32::OleCreatePictureIndirectExt(emu),
        "OleCreatePropertyFrameIndirectExt" => ole32::OleCreatePropertyFrameIndirectExt(emu),
        "OleCreateStaticFromData" => ole32::OleCreateStaticFromData(emu),
        "OleDestroyMenuDescriptor" => ole32::OleDestroyMenuDescriptor(emu),
        "OleDoAutoConvert" => ole32::OleDoAutoConvert(emu),
        "OleDraw" => ole32::OleDraw(emu),
        "OleDuplicateData" => ole32::OleDuplicateData(emu),
        "OleFlushClipboard" => ole32::OleFlushClipboard(emu),
        "OleGetAutoConvert" => ole32::OleGetAutoConvert(emu),
        "OleGetClipboard" => ole32::OleGetClipboard(emu),
        "OleGetClipboardWithEnterpriseInfo" => ole32::OleGetClipboardWithEnterpriseInfo(emu),
        "OleGetIconOfClass" => ole32::OleGetIconOfClass(emu),
        "OleGetIconOfFile" => ole32::OleGetIconOfFile(emu),
        "OleGetPackageClipboardOwner" => ole32::OleGetPackageClipboardOwner(emu),
        "OleIconToCursorExt" => ole32::OleIconToCursorExt(emu),
        "OleIsCurrentClipboard" => ole32::OleIsCurrentClipboard(emu),
        "OleIsRunning" => ole32::OleIsRunning(emu),
        "OleLoad" => ole32::OleLoad(emu),
        "OleLoadPictureExt" => ole32::OleLoadPictureExt(emu),
        "OleLoadPictureFileExt" => ole32::OleLoadPictureFileExt(emu),
        "OleLoadPicturePathExt" => ole32::OleLoadPicturePathExt(emu),
        "OleLockRunning" => ole32::OleLockRunning(emu),
        "OleMetafilePictFromIconAndLabel" => ole32::OleMetafilePictFromIconAndLabel(emu),
        "OleNoteObjectVisible" => ole32::OleNoteObjectVisible(emu),
        "OleQueryCreateFromData" => ole32::OleQueryCreateFromData(emu),
        "OleQueryLinkFromData" => ole32::OleQueryLinkFromData(emu),
        "OleRegEnumFormatEtc" => ole32::OleRegEnumFormatEtc(emu),
        "OleRegEnumVerbs" => ole32::OleRegEnumVerbs(emu),
        "OleRegGetMiscStatus" => ole32::OleRegGetMiscStatus(emu),
        "OleRegGetUserType" => ole32::OleRegGetUserType(emu),
        "OleReleaseEnumVerbCache" => ole32::OleReleaseEnumVerbCache(emu),
        "OleRun" => ole32::OleRun(emu),
        "OleSave" => ole32::OleSave(emu),
        "OleSavePictureFileExt" => ole32::OleSavePictureFileExt(emu),
        "OleSetAutoConvert" => ole32::OleSetAutoConvert(emu),
        "OleSetClipboard" => ole32::OleSetClipboard(emu),
        "OleSetContainedObject" => ole32::OleSetContainedObject(emu),
        "OleSetMenuDescriptor" => ole32::OleSetMenuDescriptor(emu),
        "OleTranslateAccelerator" => ole32::OleTranslateAccelerator(emu),
        "OleTranslateColorExt" => ole32::OleTranslateColorExt(emu),
        "RegisterDragDrop" => ole32::RegisterDragDrop(emu),
        "RevokeDragDrop" => ole32::RevokeDragDrop(emu),
        "UtConvertDvtd16toDvtd32" => ole32::UtConvertDvtd16toDvtd32(emu),
        "UtConvertDvtd32toDvtd16" => ole32::UtConvertDvtd32toDvtd16(emu),
        "UtGetDvtd16Info" => ole32::UtGetDvtd16Info(emu),
        "UtGetDvtd32Info" => ole32::UtGetDvtd32Info(emu),
        _ => return false,
    }
    true
}
