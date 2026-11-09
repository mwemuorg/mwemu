use crate::emu::Emu;
use crate::winapi::winapi32::ole32;

#[path = "../bind_moniker.rs"]
mod bind_moniker;
pub use bind_moniker::BindMoniker;

#[path = "../check_init_dde.rs"]
mod check_init_dde;
pub use check_init_dde::CheckInitDde;

#[path = "../clean_rot_for_apartment.rs"]
mod clean_rot_for_apartment;
pub use clean_rot_for_apartment::CleanROTForApartment;

#[path = "../clipboard_process_uninitialize.rs"]
mod clipboard_process_uninitialize;
pub use clipboard_process_uninitialize::ClipboardProcessUninitialize;

#[path = "../create_anti_moniker.rs"]
mod create_anti_moniker;
pub use create_anti_moniker::CreateAntiMoniker;

#[path = "../create_bind_ctx.rs"]
mod create_bind_ctx;
pub use create_bind_ctx::CreateBindCtx;

#[path = "../create_class_moniker.rs"]
mod create_class_moniker;
pub use create_class_moniker::CreateClassMoniker;

#[path = "../create_data_advise_holder.rs"]
mod create_data_advise_holder;
pub use create_data_advise_holder::CreateDataAdviseHolder;

#[path = "../create_data_cache.rs"]
mod create_data_cache;
pub use create_data_cache::CreateDataCache;

#[path = "../create_file_moniker.rs"]
mod create_file_moniker;
pub use create_file_moniker::CreateFileMoniker;

#[path = "../create_generic_composite.rs"]
mod create_generic_composite;
pub use create_generic_composite::CreateGenericComposite;

#[path = "../create_item_moniker.rs"]
mod create_item_moniker;
pub use create_item_moniker::CreateItemMoniker;

#[path = "../create_objref_moniker.rs"]
mod create_objref_moniker;
pub use create_objref_moniker::CreateObjrefMoniker;

#[path = "../create_ole_advise_holder.rs"]
mod create_ole_advise_holder;
pub use create_ole_advise_holder::CreateOleAdviseHolder;

#[path = "../create_pointer_moniker.rs"]
mod create_pointer_moniker;
pub use create_pointer_moniker::CreatePointerMoniker;

#[path = "../dde_bind_to_object.rs"]
mod dde_bind_to_object;
pub use dde_bind_to_object::DdeBindToObject;

#[path = "../destroy_running_object_table.rs"]
mod destroy_running_object_table;
pub use destroy_running_object_table::DestroyRunningObjectTable;

#[path = "../do_drag_drop.rs"]
mod do_drag_drop;
pub use do_drag_drop::DoDragDrop;

#[path = "../drag_drop_set_fdt.rs"]
mod drag_drop_set_fdt;
pub use drag_drop_set_fdt::DragDropSetFDT;

#[path = "../get_object_from_rot_by_path.rs"]
mod get_object_from_rot_by_path;
pub use get_object_from_rot_by_path::GetObjectFromRotByPath;

#[path = "../get_running_object_table.rs"]
mod get_running_object_table;
pub use get_running_object_table::GetRunningObjectTable;

#[path = "../hk_ole_register_object.rs"]
mod hk_ole_register_object;
pub use hk_ole_register_object::HkOleRegisterObject;

#[path = "../is_accelerator.rs"]
mod is_accelerator;
pub use is_accelerator::IsAccelerator;

#[path = "../moniker_common_prefix_with.rs"]
mod moniker_common_prefix_with;
pub use moniker_common_prefix_with::MonikerCommonPrefixWith;

#[path = "../moniker_load_type_lib.rs"]
mod moniker_load_type_lib;
pub use moniker_load_type_lib::MonikerLoadTypeLib;

#[path = "../moniker_relative_path_to.rs"]
mod moniker_relative_path_to;
pub use moniker_relative_path_to::MonikerRelativePathTo;

#[path = "../ole32_dll_get_class_object.rs"]
mod ole32_dll_get_class_object;
pub use ole32_dll_get_class_object::Ole32DllGetClassObject;

#[path = "../ole_build_version.rs"]
mod ole_build_version;
pub use ole_build_version::OleBuildVersion;

#[path = "../ole_create.rs"]
mod ole_create;
pub use ole_create::OleCreate;

#[path = "../ole_create_default_handler.rs"]
mod ole_create_default_handler;
pub use ole_create_default_handler::OleCreateDefaultHandler;

#[path = "../ole_create_embedding_helper.rs"]
mod ole_create_embedding_helper;
pub use ole_create_embedding_helper::OleCreateEmbeddingHelper;

#[path = "../ole_create_ex.rs"]
mod ole_create_ex;
pub use ole_create_ex::OleCreateEx;

#[path = "../ole_create_font_indirect_ext.rs"]
mod ole_create_font_indirect_ext;
pub use ole_create_font_indirect_ext::OleCreateFontIndirectExt;

#[path = "../ole_create_from_data.rs"]
mod ole_create_from_data;
pub use ole_create_from_data::OleCreateFromData;

#[path = "../ole_create_from_data_ex.rs"]
mod ole_create_from_data_ex;
pub use ole_create_from_data_ex::OleCreateFromDataEx;

#[path = "../ole_create_from_file.rs"]
mod ole_create_from_file;
pub use ole_create_from_file::OleCreateFromFile;

#[path = "../ole_create_from_file_ex.rs"]
mod ole_create_from_file_ex;
pub use ole_create_from_file_ex::OleCreateFromFileEx;

#[path = "../ole_create_link.rs"]
mod ole_create_link;
pub use ole_create_link::OleCreateLink;

#[path = "../ole_create_link_ex.rs"]
mod ole_create_link_ex;
pub use ole_create_link_ex::OleCreateLinkEx;

#[path = "../ole_create_link_from_data.rs"]
mod ole_create_link_from_data;
pub use ole_create_link_from_data::OleCreateLinkFromData;

#[path = "../ole_create_link_from_data_ex.rs"]
mod ole_create_link_from_data_ex;
pub use ole_create_link_from_data_ex::OleCreateLinkFromDataEx;

#[path = "../ole_create_link_to_file.rs"]
mod ole_create_link_to_file;
pub use ole_create_link_to_file::OleCreateLinkToFile;

#[path = "../ole_create_link_to_file_ex.rs"]
mod ole_create_link_to_file_ex;
pub use ole_create_link_to_file_ex::OleCreateLinkToFileEx;

#[path = "../ole_create_menu_descriptor.rs"]
mod ole_create_menu_descriptor;
pub use ole_create_menu_descriptor::OleCreateMenuDescriptor;

#[path = "../ole_create_picture_indirect_ext.rs"]
mod ole_create_picture_indirect_ext;
pub use ole_create_picture_indirect_ext::OleCreatePictureIndirectExt;

#[path = "../ole_create_property_frame_indirect_ext.rs"]
mod ole_create_property_frame_indirect_ext;
pub use ole_create_property_frame_indirect_ext::OleCreatePropertyFrameIndirectExt;

#[path = "../ole_create_static_from_data.rs"]
mod ole_create_static_from_data;
pub use ole_create_static_from_data::OleCreateStaticFromData;

#[path = "../ole_destroy_menu_descriptor.rs"]
mod ole_destroy_menu_descriptor;
pub use ole_destroy_menu_descriptor::OleDestroyMenuDescriptor;

#[path = "../ole_do_auto_convert.rs"]
mod ole_do_auto_convert;
pub use ole_do_auto_convert::OleDoAutoConvert;

#[path = "../ole_draw.rs"]
mod ole_draw;
pub use ole_draw::OleDraw;

#[path = "../ole_duplicate_data.rs"]
mod ole_duplicate_data;
pub use ole_duplicate_data::OleDuplicateData;

#[path = "../ole_flush_clipboard.rs"]
mod ole_flush_clipboard;
pub use ole_flush_clipboard::OleFlushClipboard;

#[path = "../ole_get_auto_convert.rs"]
mod ole_get_auto_convert;
pub use ole_get_auto_convert::OleGetAutoConvert;

#[path = "../ole_get_clipboard.rs"]
mod ole_get_clipboard;
pub use ole_get_clipboard::OleGetClipboard;

#[path = "../ole_get_clipboard_with_enterprise_info.rs"]
mod ole_get_clipboard_with_enterprise_info;
pub use ole_get_clipboard_with_enterprise_info::OleGetClipboardWithEnterpriseInfo;

#[path = "../ole_get_icon_of_class.rs"]
mod ole_get_icon_of_class;
pub use ole_get_icon_of_class::OleGetIconOfClass;

#[path = "../ole_get_icon_of_file.rs"]
mod ole_get_icon_of_file;
pub use ole_get_icon_of_file::OleGetIconOfFile;

#[path = "../ole_get_package_clipboard_owner.rs"]
mod ole_get_package_clipboard_owner;
pub use ole_get_package_clipboard_owner::OleGetPackageClipboardOwner;

#[path = "../ole_icon_to_cursor_ext.rs"]
mod ole_icon_to_cursor_ext;
pub use ole_icon_to_cursor_ext::OleIconToCursorExt;

#[path = "../ole_initialize_wow.rs"]
mod ole_initialize_wow;
pub use ole_initialize_wow::OleInitializeWOW;

#[path = "../ole_is_current_clipboard.rs"]
mod ole_is_current_clipboard;
pub use ole_is_current_clipboard::OleIsCurrentClipboard;

#[path = "../ole_is_running.rs"]
mod ole_is_running;
pub use ole_is_running::OleIsRunning;

#[path = "../ole_load.rs"]
mod ole_load;
pub use ole_load::OleLoad;

#[path = "../ole_load_picture_ext.rs"]
mod ole_load_picture_ext;
pub use ole_load_picture_ext::OleLoadPictureExt;

#[path = "../ole_load_picture_file_ext.rs"]
mod ole_load_picture_file_ext;
pub use ole_load_picture_file_ext::OleLoadPictureFileExt;

#[path = "../ole_load_picture_path_ext.rs"]
mod ole_load_picture_path_ext;
pub use ole_load_picture_path_ext::OleLoadPicturePathExt;

#[path = "../ole_lock_running.rs"]
mod ole_lock_running;
pub use ole_lock_running::OleLockRunning;

#[path = "../ole_metafile_pict_from_icon_and_label.rs"]
mod ole_metafile_pict_from_icon_and_label;
pub use ole_metafile_pict_from_icon_and_label::OleMetafilePictFromIconAndLabel;

#[path = "../ole_note_object_visible.rs"]
mod ole_note_object_visible;
pub use ole_note_object_visible::OleNoteObjectVisible;

#[path = "../ole_query_create_from_data.rs"]
mod ole_query_create_from_data;
pub use ole_query_create_from_data::OleQueryCreateFromData;

#[path = "../ole_query_link_from_data.rs"]
mod ole_query_link_from_data;
pub use ole_query_link_from_data::OleQueryLinkFromData;

#[path = "../ole_reg_enum_format_etc.rs"]
mod ole_reg_enum_format_etc;
pub use ole_reg_enum_format_etc::OleRegEnumFormatEtc;

#[path = "../ole_reg_enum_verbs.rs"]
mod ole_reg_enum_verbs;
pub use ole_reg_enum_verbs::OleRegEnumVerbs;

#[path = "../ole_reg_get_misc_status.rs"]
mod ole_reg_get_misc_status;
pub use ole_reg_get_misc_status::OleRegGetMiscStatus;

#[path = "../ole_reg_get_user_type.rs"]
mod ole_reg_get_user_type;
pub use ole_reg_get_user_type::OleRegGetUserType;

#[path = "../ole_release_enum_verb_cache.rs"]
mod ole_release_enum_verb_cache;
pub use ole_release_enum_verb_cache::OleReleaseEnumVerbCache;

#[path = "../ole_run.rs"]
mod ole_run;
pub use ole_run::OleRun;

#[path = "../ole_save.rs"]
mod ole_save;
pub use ole_save::OleSave;

#[path = "../ole_save_picture_file_ext.rs"]
mod ole_save_picture_file_ext;
pub use ole_save_picture_file_ext::OleSavePictureFileExt;

#[path = "../ole_set_auto_convert.rs"]
mod ole_set_auto_convert;
pub use ole_set_auto_convert::OleSetAutoConvert;

#[path = "../ole_set_clipboard.rs"]
mod ole_set_clipboard;
pub use ole_set_clipboard::OleSetClipboard;

#[path = "../ole_set_contained_object.rs"]
mod ole_set_contained_object;
pub use ole_set_contained_object::OleSetContainedObject;

#[path = "../ole_set_menu_descriptor.rs"]
mod ole_set_menu_descriptor;
pub use ole_set_menu_descriptor::OleSetMenuDescriptor;

#[path = "../ole_translate_accelerator.rs"]
mod ole_translate_accelerator;
pub use ole_translate_accelerator::OleTranslateAccelerator;

#[path = "../ole_translate_color_ext.rs"]
mod ole_translate_color_ext;
pub use ole_translate_color_ext::OleTranslateColorExt;

#[path = "../ole_uninitialize.rs"]
mod ole_uninitialize;
pub use ole_uninitialize::OleUninitialize;

#[path = "../register_drag_drop.rs"]
mod register_drag_drop;
pub use register_drag_drop::RegisterDragDrop;

#[path = "../revoke_drag_drop.rs"]
mod revoke_drag_drop;
pub use revoke_drag_drop::RevokeDragDrop;

#[path = "../ut_convert_dvtd16to_dvtd32.rs"]
mod ut_convert_dvtd16to_dvtd32;
pub use ut_convert_dvtd16to_dvtd32::UtConvertDvtd16toDvtd32;

#[path = "../ut_convert_dvtd32to_dvtd16.rs"]
mod ut_convert_dvtd32to_dvtd16;
pub use ut_convert_dvtd32to_dvtd16::UtConvertDvtd32toDvtd16;

#[path = "../ut_get_dvtd16_info.rs"]
mod ut_get_dvtd16_info;
pub use ut_get_dvtd16_info::UtGetDvtd16Info;

#[path = "../ut_get_dvtd32_info.rs"]
mod ut_get_dvtd32_info;
pub use ut_get_dvtd32_info::UtGetDvtd32Info;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "BindMoniker" => ole32::BindMoniker(emu),
        "CheckInitDde" => ole32::CheckInitDde(emu),
        "CleanROTForApartment" => ole32::CleanROTForApartment(emu),
        "ClipboardProcessUninitialize" => ole32::ClipboardProcessUninitialize(emu),
        "CreateAntiMoniker" => ole32::CreateAntiMoniker(emu),
        "CreateBindCtx" => ole32::CreateBindCtx(emu),
        "CreateClassMoniker" => ole32::CreateClassMoniker(emu),
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
        "GetRunningObjectTable" => ole32::GetRunningObjectTable(emu),
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
