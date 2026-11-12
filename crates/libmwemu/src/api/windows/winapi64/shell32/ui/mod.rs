use crate::emu::Emu;
use crate::winapi::winapi64::shell32;

mod sh_update_image;
pub use sh_update_image::{SHUpdateImageA, SHUpdateImageW};

mod sh_get_icon_overlay_index;
pub use sh_get_icon_overlay_index::{SHGetIconOverlayIndexA, SHGetIconOverlayIndexW};

mod sh_def_extract_icon;
pub use sh_def_extract_icon::{SHDefExtractIconA, SHDefExtractIconW};

mod internal_extract_icon_list;
pub use internal_extract_icon_list::{InternalExtractIconListA, InternalExtractIconListW};

mod extract_icon;
pub use extract_icon::{ExtractIconA, ExtractIconW};

mod extract_associated_icon_ex;
pub use extract_associated_icon_ex::{ExtractAssociatedIconExA, ExtractAssociatedIconExW};

mod extract_associated_icon;
pub use extract_associated_icon::{ExtractAssociatedIconA, ExtractAssociatedIconW};

mod c_def_folder_menu_create2;
pub use c_def_folder_menu_create2::CDefFolderMenu_Create2;

mod dad_auto_scroll;
pub use dad_auto_scroll::DAD_AutoScroll;

mod dad_drag_enter_ex;
pub use dad_drag_enter_ex::DAD_DragEnterEx;

mod dad_drag_enter_ex2;
pub use dad_drag_enter_ex2::DAD_DragEnterEx2;

mod dad_drag_leave;
pub use dad_drag_leave::DAD_DragLeave;

mod dad_drag_move;
pub use dad_drag_move::DAD_DragMove;

mod dad_set_drag_image;
pub use dad_set_drag_image::DAD_SetDragImage;

mod dad_show_drag_image;
pub use dad_show_drag_image::DAD_ShowDragImage;

mod drag_accept_files;
pub use drag_accept_files::DragAcceptFiles;

mod drag_finish;
pub use drag_finish::DragFinish;

mod drag_query_file;
pub use drag_query_file::{DragQueryFile, DragQueryFileA, DragQueryFileW};

mod drag_query_file_aor_w;
pub use drag_query_file_aor_w::DragQueryFileAorW;

mod drag_query_point;
pub use drag_query_point::DragQueryPoint;

mod duplicate_icon;
pub use duplicate_icon::DuplicateIcon;

mod extract_icon_ex;
pub use extract_icon_ex::{ExtractIconEx, ExtractIconExA, ExtractIconExW};

mod free_icon_list;
pub use free_icon_list::FreeIconList;

mod get_file_name_from_browse;
pub use get_file_name_from_browse::GetFileNameFromBrowse;

mod pick_icon_dlg;
pub use pick_icon_dlg::PickIconDlg;

mod restart_dialog;
pub use restart_dialog::RestartDialog;

mod restart_dialog_ex;
pub use restart_dialog_ex::RestartDialogEx;

mod sh_add_from_prop_sheet_ext_array;
pub use sh_add_from_prop_sheet_ext_array::SHAddFromPropSheetExtArray;

mod sh_app_bar_message;
pub use sh_app_bar_message::SHAppBarMessage;

mod sh_browse_for_folder;
pub use sh_browse_for_folder::{SHBrowseForFolder, SHBrowseForFolderA, SHBrowseForFolderW};

mod sh_create_default_context_menu;
pub use sh_create_default_context_menu::SHCreateDefaultContextMenu;

mod sh_create_default_extract_icon;
pub use sh_create_default_extract_icon::SHCreateDefaultExtractIcon;

mod sh_create_drv_ext_icon;
pub use sh_create_drv_ext_icon::SHCreateDrvExtIcon;

mod sh_create_file_extract_icon_w;
pub use sh_create_file_extract_icon_w::SHCreateFileExtractIconW;

mod sh_create_prop_sheet_ext_array;
pub use sh_create_prop_sheet_ext_array::SHCreatePropSheetExtArray;

mod sh_destroy_prop_sheet_ext_array;
pub use sh_destroy_prop_sheet_ext_array::SHDestroyPropSheetExtArray;

mod sh_do_drag_drop;
pub use sh_do_drag_drop::SHDoDragDrop;

mod shell32_add_to_back_icon_table;
pub use shell32_add_to_back_icon_table::SHELL32_AddToBackIconTable;

mod shell32_add_to_front_icon_table;
pub use shell32_add_to_front_icon_table::SHELL32_AddToFrontIconTable;

mod shell32_c_def_folder_menu_create2;
pub use shell32_c_def_folder_menu_create2::SHELL32_CDefFolderMenu_Create2;

mod shell32_c_def_folder_menu_create2_ex;
pub use shell32_c_def_folder_menu_create2_ex::SHELL32_CDefFolderMenu_Create2Ex;

mod shell32_c_def_folder_menu_merge_menu;
pub use shell32_c_def_folder_menu_merge_menu::SHELL32_CDefFolderMenu_MergeMenu;

mod shell32_c_drives_context_menu_create;
pub use shell32_c_drives_context_menu_create::SHELL32_CDrivesContextMenu_Create;

mod shell32_c_drives_drop_target_create;
pub use shell32_c_drives_drop_target_create::SHELL32_CDrivesDropTarget_Create;

mod shell32_cfs_drop_target_create_instance;
pub use shell32_cfs_drop_target_create_instance::SHELL32_CFSDropTarget_CreateInstance;

mod shell32_c_library_drop_target_create_instance;
pub use shell32_c_library_drop_target_create_instance::SHELL32_CLibraryDropTarget_CreateInstance;

mod shell32_c_location_context_menu_create;
pub use shell32_c_location_context_menu_create::SHELL32_CLocationContextMenu_Create;

mod shell32_c_location_folder_ui_create_instance;
pub use shell32_c_location_folder_ui_create_instance::SHELL32_CLocationFolderUI_CreateInstance;

mod shell32_c_mount_point_want_autorun_ui;
pub use shell32_c_mount_point_want_autorun_ui::SHELL32_CMountPoint_WantAutorunUI;

mod shell32_c_mount_point_want_autorun_ui_get_ready;
pub use shell32_c_mount_point_want_autorun_ui_get_ready::SHELL32_CMountPoint_WantAutorunUIGetReady;

mod shell32_c_net_folder_ui_create_instance;
pub use shell32_c_net_folder_ui_create_instance::SHELL32_CNetFolderUI_CreateInstance;

mod shell32_c_recent_docs_context_menu_create_instance;
pub use shell32_c_recent_docs_context_menu_create_instance::SHELL32_CRecentDocsContextMenu_CreateInstance;

mod shell32_can_display_win8_copy_dialog;
pub use shell32_can_display_win8_copy_dialog::SHELL32_CanDisplayWin8CopyDialog;

mod shell32_create_file_folder_context_menu;
pub use shell32_create_file_folder_context_menu::SHELL32_CreateFileFolderContextMenu;

mod shell32_create_i_enum_ui_command;
pub use shell32_create_i_enum_ui_command::SHELL32_Create_IEnumUICommand;

mod shell32_get_file_name_from_browse;
pub use shell32_get_file_name_from_browse::SHELL32_GetFileNameFromBrowse;

mod shell32_get_icon_overlay_manager;
pub use shell32_get_icon_overlay_manager::SHELL32_GetIconOverlayManager;

mod shell32_get_thumbnail_adorner_from_factory;
pub use shell32_get_thumbnail_adorner_from_factory::SHELL32_GetThumbnailAdornerFromFactory;

mod shell32_get_thumbnail_adorner_from_factory2;
pub use shell32_get_thumbnail_adorner_from_factory2::SHELL32_GetThumbnailAdornerFromFactory2;

mod shell32_icon_cache_create;
pub use shell32_icon_cache_create::SHELL32_IconCacheCreate;

mod shell32_icon_cache_destroy;
pub use shell32_icon_cache_destroy::SHELL32_IconCacheDestroy;

mod shell32_icon_cache_handle_association_changed;
pub use shell32_icon_cache_handle_association_changed::SHELL32_IconCacheHandleAssociationChanged;

mod shell32_icon_cache_restore;
pub use shell32_icon_cache_restore::SHELL32_IconCacheRestore;

mod shell32_icon_cache_about_to_extract_icons;
pub use shell32_icon_cache_about_to_extract_icons::SHELL32_IconCache_AboutToExtractIcons;

mod shell32_icon_cache_done_extracting_icons;
pub use shell32_icon_cache_done_extracting_icons::SHELL32_IconCache_DoneExtractingIcons;

mod shell32_icon_cache_expand_env_and_search_path;
pub use shell32_icon_cache_expand_env_and_search_path::SHELL32_IconCache_ExpandEnvAndSearchPath;

mod shell32_icon_cache_remember_recently_extracted_icons_w;
pub use shell32_icon_cache_remember_recently_extracted_icons_w::SHELL32_IconCache_RememberRecentlyExtractedIconsW;

mod shell32_icon_overlay_manager_init;
pub use shell32_icon_overlay_manager_init::SHELL32_IconOverlayManagerInit;

mod shell32_lookup_back_icon_index;
pub use shell32_lookup_back_icon_index::SHELL32_LookupBackIconIndex;

mod shell32_lookup_front_icon_index;
pub use shell32_lookup_front_icon_index::SHELL32_LookupFrontIconIndex;

mod shell32_purge_system_icon;
pub use shell32_purge_system_icon::SHELL32_PurgeSystemIcon;

mod shell32_refresh_overlay_images;
pub use shell32_refresh_overlay_images::SHELL32_RefreshOverlayImages;

mod shell32_sh_add_sparse_icon;
pub use shell32_sh_add_sparse_icon::SHELL32_SHAddSparseIcon;

mod shell32_sh_create_default_context_menu;
pub use shell32_sh_create_default_context_menu::SHELL32_SHCreateDefaultContextMenu;

mod shell32_sh_launch_prop_sheet;
pub use shell32_sh_launch_prop_sheet::SHELL32_SHLaunchPropSheet;

mod shell32_sh_open_with_dialog;
pub use shell32_sh_open_with_dialog::SHELL32_SHOpenWithDialog;

mod shell32_sh_start_net_connection_dialog_w;
pub use shell32_sh_start_net_connection_dialog_w::SHELL32_SHStartNetConnectionDialogW;

mod shell32_shui_command_from_guid;
pub use shell32_shui_command_from_guid::SHELL32_SHUICommandFromGUID;

mod shell32_send_to_menu_invoke_targeted_command;
pub use shell32_send_to_menu_invoke_targeted_command::SHELL32_SendToMenu_InvokeTargetedCommand;

mod shell32_send_to_menu_verify_targeted_command;
pub use shell32_send_to_menu_verify_targeted_command::SHELL32_SendToMenu_VerifyTargetedCommand;

mod shell32_show_hide_icon_only_on_desktop;
pub use shell32_show_hide_icon_only_on_desktop::SHELL32_ShowHideIconOnlyOnDesktop;

mod shell32_stamp_icon_for_file;
pub use shell32_stamp_icon_for_file::SHELL32_StampIconForFile;

mod shell32_try_virtual_disc_image_drive_eject;
pub use shell32_try_virtual_disc_image_drive_eject::SHELL32_TryVirtualDiscImageDriveEject;

mod sh_extract_icons_w;
pub use sh_extract_icons_w::SHExtractIconsW;

mod sh_find_init_menu_popup;
pub use sh_find_init_menu_popup::SHFind_InitMenuPopup;

mod sh_get_image_list;
pub use sh_get_image_list::SHGetImageList;

mod sh_get_property_store_for_window;
pub use sh_get_property_store_for_window::SHGetPropertyStoreForWindow;

mod sh_get_stock_icon_info;
pub use sh_get_stock_icon_info::SHGetStockIconInfo;

mod sh_handle_update_image;
pub use sh_handle_update_image::SHHandleUpdateImage;

mod sh_load_nonloaded_icon_overlay_identifiers;
pub use sh_load_nonloaded_icon_overlay_identifiers::SHLoadNonloadedIconOverlayIdentifiers;

mod sh_map_pidl_to_system_image_list_index;
pub use sh_map_pidl_to_system_image_list_index::SHMapPIDLToSystemImageListIndex;

mod sh_open_prop_sheet_w;
pub use sh_open_prop_sheet_w::SHOpenPropSheetW;

mod sh_open_with_dialog;
pub use sh_open_with_dialog::SHOpenWithDialog;

mod sh_replace_from_prop_sheet_ext_array;
pub use sh_replace_from_prop_sheet_ext_array::SHReplaceFromPropSheetExtArray;

mod sh_show_manage_library_ui;
pub use sh_show_manage_library_ui::SHShowManageLibraryUI;

mod sh_start_net_connection_dialog_w;
pub use sh_start_net_connection_dialog_w::SHStartNetConnectionDialogW;

mod sh_update_recycle_bin_icon;
pub use sh_update_recycle_bin_icon::SHUpdateRecycleBinIcon;

mod shell_get_cached_image_index;
pub use shell_get_cached_image_index::{
    Shell_GetCachedImageIndex, Shell_GetCachedImageIndexA, Shell_GetCachedImageIndexW,
};

mod shell_get_image_lists;
pub use shell_get_image_lists::Shell_GetImageLists;

mod shell_merge_menus;
pub use shell_merge_menus::Shell_MergeMenus;

mod shell_notify_icon;
pub use shell_notify_icon::{Shell_NotifyIcon, Shell_NotifyIconA, Shell_NotifyIconW};

mod shell_notify_icon_get_rect;
pub use shell_notify_icon_get_rect::Shell_NotifyIconGetRect;

mod state_repo_new_menu_cache_ensure_cache_async;
pub use state_repo_new_menu_cache_ensure_cache_async::StateRepoNewMenuCache_EnsureCacheAsync;

mod state_repo_new_menu_cache_rebuild_cache_async;
pub use state_repo_new_menu_cache_rebuild_cache_async::StateRepoNewMenuCache_RebuildCacheAsync;

mod users_libraries_folder_ui_create_instance;
pub use users_libraries_folder_ui_create_instance::UsersLibrariesFolderUI_CreateInstance;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "CDefFolderMenu_Create2" => shell32::CDefFolderMenu_Create2(emu),
        "DAD_AutoScroll" => shell32::DAD_AutoScroll(emu),
        "DAD_DragEnterEx" => shell32::DAD_DragEnterEx(emu),
        "DAD_DragEnterEx2" => shell32::DAD_DragEnterEx2(emu),
        "DAD_DragLeave" => shell32::DAD_DragLeave(emu),
        "DAD_DragMove" => shell32::DAD_DragMove(emu),
        "DAD_SetDragImage" => shell32::DAD_SetDragImage(emu),
        "DAD_ShowDragImage" => shell32::DAD_ShowDragImage(emu),
        "DragAcceptFiles" => shell32::DragAcceptFiles(emu),
        "DragFinish" => shell32::DragFinish(emu),
        "DragQueryFile" => shell32::DragQueryFile(emu),
        "DragQueryFileA" => shell32::DragQueryFileA(emu),
        "DragQueryFileAorW" => shell32::DragQueryFileAorW(emu),
        "DragQueryFileW" => shell32::DragQueryFileW(emu),
        "DragQueryPoint" => shell32::DragQueryPoint(emu),
        "DuplicateIcon" => shell32::DuplicateIcon(emu),
        "ExtractAssociatedIconA" => shell32::ExtractAssociatedIconA(emu),
        "ExtractAssociatedIconExA" => shell32::ExtractAssociatedIconExA(emu),
        "ExtractAssociatedIconExW" => shell32::ExtractAssociatedIconExW(emu),
        "ExtractAssociatedIconW" => shell32::ExtractAssociatedIconW(emu),
        "ExtractIconA" => shell32::ExtractIconA(emu),
        "ExtractIconEx" => shell32::ExtractIconEx(emu),
        "ExtractIconExA" => shell32::ExtractIconExA(emu),
        "ExtractIconExW" => shell32::ExtractIconExW(emu),
        "ExtractIconW" => shell32::ExtractIconW(emu),
        "FreeIconList" => shell32::FreeIconList(emu),
        "GetFileNameFromBrowse" => shell32::GetFileNameFromBrowse(emu),
        "InternalExtractIconListA" => shell32::InternalExtractIconListA(emu),
        "InternalExtractIconListW" => shell32::InternalExtractIconListW(emu),
        "PickIconDlg" => shell32::PickIconDlg(emu),
        "RestartDialog" => shell32::RestartDialog(emu),
        "RestartDialogEx" => shell32::RestartDialogEx(emu),
        "SHAddFromPropSheetExtArray" => shell32::SHAddFromPropSheetExtArray(emu),
        "SHAppBarMessage" => shell32::SHAppBarMessage(emu),
        "SHBrowseForFolder" => shell32::SHBrowseForFolder(emu),
        "SHBrowseForFolderA" => shell32::SHBrowseForFolderA(emu),
        "SHBrowseForFolderW" => shell32::SHBrowseForFolderW(emu),
        "SHCreateDefaultContextMenu" => shell32::SHCreateDefaultContextMenu(emu),
        "SHCreateDefaultExtractIcon" => shell32::SHCreateDefaultExtractIcon(emu),
        "SHCreateDrvExtIcon" => shell32::SHCreateDrvExtIcon(emu),
        "SHCreateFileExtractIconW" => shell32::SHCreateFileExtractIconW(emu),
        "SHCreatePropSheetExtArray" => shell32::SHCreatePropSheetExtArray(emu),
        "SHDefExtractIconA" => shell32::SHDefExtractIconA(emu),
        "SHDefExtractIconW" => shell32::SHDefExtractIconW(emu),
        "SHDestroyPropSheetExtArray" => shell32::SHDestroyPropSheetExtArray(emu),
        "SHDoDragDrop" => shell32::SHDoDragDrop(emu),
        "SHELL32_AddToBackIconTable" => shell32::SHELL32_AddToBackIconTable(emu),
        "SHELL32_AddToFrontIconTable" => shell32::SHELL32_AddToFrontIconTable(emu),
        "SHELL32_CDefFolderMenu_Create2" => shell32::SHELL32_CDefFolderMenu_Create2(emu),
        "SHELL32_CDefFolderMenu_Create2Ex" => shell32::SHELL32_CDefFolderMenu_Create2Ex(emu),
        "SHELL32_CDefFolderMenu_MergeMenu" => shell32::SHELL32_CDefFolderMenu_MergeMenu(emu),
        "SHELL32_CDrivesContextMenu_Create" => shell32::SHELL32_CDrivesContextMenu_Create(emu),
        "SHELL32_CDrivesDropTarget_Create" => shell32::SHELL32_CDrivesDropTarget_Create(emu),
        "SHELL32_CFSDropTarget_CreateInstance" => {
            shell32::SHELL32_CFSDropTarget_CreateInstance(emu)
        }
        "SHELL32_CLibraryDropTarget_CreateInstance" => {
            shell32::SHELL32_CLibraryDropTarget_CreateInstance(emu)
        }
        "SHELL32_CLocationContextMenu_Create" => shell32::SHELL32_CLocationContextMenu_Create(emu),
        "SHELL32_CLocationFolderUI_CreateInstance" => {
            shell32::SHELL32_CLocationFolderUI_CreateInstance(emu)
        }
        "SHELL32_CMountPoint_WantAutorunUI" => shell32::SHELL32_CMountPoint_WantAutorunUI(emu),
        "SHELL32_CMountPoint_WantAutorunUIGetReady" => {
            shell32::SHELL32_CMountPoint_WantAutorunUIGetReady(emu)
        }
        "SHELL32_CNetFolderUI_CreateInstance" => shell32::SHELL32_CNetFolderUI_CreateInstance(emu),
        "SHELL32_CRecentDocsContextMenu_CreateInstance" => {
            shell32::SHELL32_CRecentDocsContextMenu_CreateInstance(emu)
        }
        "SHELL32_CanDisplayWin8CopyDialog" => shell32::SHELL32_CanDisplayWin8CopyDialog(emu),
        "SHELL32_CreateFileFolderContextMenu" => shell32::SHELL32_CreateFileFolderContextMenu(emu),
        "SHELL32_Create_IEnumUICommand" => shell32::SHELL32_Create_IEnumUICommand(emu),
        "SHELL32_GetFileNameFromBrowse" => shell32::SHELL32_GetFileNameFromBrowse(emu),
        "SHELL32_GetIconOverlayManager" => shell32::SHELL32_GetIconOverlayManager(emu),
        "SHELL32_GetThumbnailAdornerFromFactory" => {
            shell32::SHELL32_GetThumbnailAdornerFromFactory(emu)
        }
        "SHELL32_GetThumbnailAdornerFromFactory2" => {
            shell32::SHELL32_GetThumbnailAdornerFromFactory2(emu)
        }
        "SHELL32_IconCacheCreate" => shell32::SHELL32_IconCacheCreate(emu),
        "SHELL32_IconCacheDestroy" => shell32::SHELL32_IconCacheDestroy(emu),
        "SHELL32_IconCacheHandleAssociationChanged" => {
            shell32::SHELL32_IconCacheHandleAssociationChanged(emu)
        }
        "SHELL32_IconCacheRestore" => shell32::SHELL32_IconCacheRestore(emu),
        "SHELL32_IconCache_AboutToExtractIcons" => {
            shell32::SHELL32_IconCache_AboutToExtractIcons(emu)
        }
        "SHELL32_IconCache_DoneExtractingIcons" => {
            shell32::SHELL32_IconCache_DoneExtractingIcons(emu)
        }
        "SHELL32_IconCache_ExpandEnvAndSearchPath" => {
            shell32::SHELL32_IconCache_ExpandEnvAndSearchPath(emu)
        }
        "SHELL32_IconCache_RememberRecentlyExtractedIconsW" => {
            shell32::SHELL32_IconCache_RememberRecentlyExtractedIconsW(emu)
        }
        "SHELL32_IconOverlayManagerInit" => shell32::SHELL32_IconOverlayManagerInit(emu),
        "SHELL32_LookupBackIconIndex" => shell32::SHELL32_LookupBackIconIndex(emu),
        "SHELL32_LookupFrontIconIndex" => shell32::SHELL32_LookupFrontIconIndex(emu),
        "SHELL32_PurgeSystemIcon" => shell32::SHELL32_PurgeSystemIcon(emu),
        "SHELL32_RefreshOverlayImages" => shell32::SHELL32_RefreshOverlayImages(emu),
        "SHELL32_SHAddSparseIcon" => shell32::SHELL32_SHAddSparseIcon(emu),
        "SHELL32_SHCreateDefaultContextMenu" => shell32::SHELL32_SHCreateDefaultContextMenu(emu),
        "SHELL32_SHLaunchPropSheet" => shell32::SHELL32_SHLaunchPropSheet(emu),
        "SHELL32_SHOpenWithDialog" => shell32::SHELL32_SHOpenWithDialog(emu),
        "SHELL32_SHStartNetConnectionDialogW" => shell32::SHELL32_SHStartNetConnectionDialogW(emu),
        "SHELL32_SHUICommandFromGUID" => shell32::SHELL32_SHUICommandFromGUID(emu),
        "SHELL32_SendToMenu_InvokeTargetedCommand" => {
            shell32::SHELL32_SendToMenu_InvokeTargetedCommand(emu)
        }
        "SHELL32_SendToMenu_VerifyTargetedCommand" => {
            shell32::SHELL32_SendToMenu_VerifyTargetedCommand(emu)
        }
        "SHELL32_ShowHideIconOnlyOnDesktop" => shell32::SHELL32_ShowHideIconOnlyOnDesktop(emu),
        "SHELL32_StampIconForFile" => shell32::SHELL32_StampIconForFile(emu),
        "SHELL32_TryVirtualDiscImageDriveEject" => {
            shell32::SHELL32_TryVirtualDiscImageDriveEject(emu)
        }
        "SHExtractIconsW" => shell32::SHExtractIconsW(emu),
        "SHFind_InitMenuPopup" => shell32::SHFind_InitMenuPopup(emu),
        "SHGetIconOverlayIndexA" => shell32::SHGetIconOverlayIndexA(emu),
        "SHGetIconOverlayIndexW" => shell32::SHGetIconOverlayIndexW(emu),
        "SHGetImageList" => shell32::SHGetImageList(emu),
        "SHGetPropertyStoreForWindow" => shell32::SHGetPropertyStoreForWindow(emu),
        "SHGetStockIconInfo" => shell32::SHGetStockIconInfo(emu),
        "SHHandleUpdateImage" => shell32::SHHandleUpdateImage(emu),
        "SHLoadNonloadedIconOverlayIdentifiers" => {
            shell32::SHLoadNonloadedIconOverlayIdentifiers(emu)
        }
        "SHMapPIDLToSystemImageListIndex" => shell32::SHMapPIDLToSystemImageListIndex(emu),
        "SHOpenPropSheetW" => shell32::SHOpenPropSheetW(emu),
        "SHOpenWithDialog" => shell32::SHOpenWithDialog(emu),
        "SHReplaceFromPropSheetExtArray" => shell32::SHReplaceFromPropSheetExtArray(emu),
        "SHShowManageLibraryUI" => shell32::SHShowManageLibraryUI(emu),
        "SHStartNetConnectionDialogW" => shell32::SHStartNetConnectionDialogW(emu),
        "SHUpdateImageA" => shell32::SHUpdateImageA(emu),
        "SHUpdateImageW" => shell32::SHUpdateImageW(emu),
        "SHUpdateRecycleBinIcon" => shell32::SHUpdateRecycleBinIcon(emu),
        "Shell_GetCachedImageIndex" => shell32::Shell_GetCachedImageIndex(emu),
        "Shell_GetCachedImageIndexA" => shell32::Shell_GetCachedImageIndexA(emu),
        "Shell_GetCachedImageIndexW" => shell32::Shell_GetCachedImageIndexW(emu),
        "Shell_GetImageLists" => shell32::Shell_GetImageLists(emu),
        "Shell_MergeMenus" => shell32::Shell_MergeMenus(emu),
        "Shell_NotifyIcon" => shell32::Shell_NotifyIcon(emu),
        "Shell_NotifyIconA" => shell32::Shell_NotifyIconA(emu),
        "Shell_NotifyIconGetRect" => shell32::Shell_NotifyIconGetRect(emu),
        "Shell_NotifyIconW" => shell32::Shell_NotifyIconW(emu),
        "StateRepoNewMenuCache_EnsureCacheAsync" => {
            shell32::StateRepoNewMenuCache_EnsureCacheAsync(emu)
        }
        "StateRepoNewMenuCache_RebuildCacheAsync" => {
            shell32::StateRepoNewMenuCache_RebuildCacheAsync(emu)
        }
        "UsersLibrariesFolderUI_CreateInstance" => {
            shell32::UsersLibrariesFolderUI_CreateInstance(emu)
        }
        _ => return false,
    }
    true
}
