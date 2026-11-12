use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi64;
use crate::winapi::window_table;
use crate::windows::constants::errors::*;

mod wvsprintf;
pub use wvsprintf::{wvsprintfA, wvsprintfW};

mod wsprintf;
pub use wsprintf::{wsprintfA, wsprintfW};

mod win_help;
pub use win_help::{WinHelpA, WinHelpW};

mod vk_key_scan_ex;
pub use vk_key_scan_ex::{VkKeyScanExA, VkKeyScanExW};

mod vk_key_scan;
pub use vk_key_scan::{VkKeyScanA, VkKeyScanW};

mod tabbed_text_out;
pub use tabbed_text_out::{TabbedTextOutA, TabbedTextOutW};

mod set_windows_hook_ex;
pub use set_windows_hook_ex::{SetWindowsHookExA, SetWindowsHookExW};

mod set_windows_hook;
pub use set_windows_hook::{SetWindowsHookA, SetWindowsHookW};

mod set_user_object_information;
pub use set_user_object_information::{SetUserObjectInformationA, SetUserObjectInformationW};

mod set_prop;
pub use set_prop::{SetPropA, SetPropW};

mod set_menu_item_info;
pub use set_menu_item_info::{SetMenuItemInfoA, SetMenuItemInfoW};

mod set_dlg_item_text;
pub use set_dlg_item_text::{SetDlgItemTextA, SetDlgItemTextW};

mod set_class_long_ptr;
pub use set_class_long_ptr::{SetClassLongPtrA, SetClassLongPtrW};

mod set_class_long;
pub use set_class_long::{SetClassLongA, SetClassLongW};

mod send_notify_message;
pub use send_notify_message::{SendNotifyMessageA, SendNotifyMessageW};

mod send_message_timeout;
pub use send_message_timeout::{SendMessageTimeoutA, SendMessageTimeoutW};

mod send_message_callback;
pub use send_message_callback::{SendMessageCallbackA, SendMessageCallbackW};

mod send_ime_message_ex;
pub use send_ime_message_ex::{SendIMEMessageExA, SendIMEMessageExW};

mod send_dlg_item_message;
pub use send_dlg_item_message::{SendDlgItemMessageA, SendDlgItemMessageW};

mod remove_prop;
pub use remove_prop::{RemovePropA, RemovePropW};

mod register_window_message;
pub use register_window_message::{RegisterWindowMessageA, RegisterWindowMessageW};

mod register_device_notification;
pub use register_device_notification::{RegisterDeviceNotificationA, RegisterDeviceNotificationW};

mod register_clipboard_format;
pub use register_clipboard_format::{RegisterClipboardFormatA, RegisterClipboardFormatW};

mod private_extract_icons;
pub use private_extract_icons::{PrivateExtractIconsA, PrivateExtractIconsW};

mod private_extract_icon_ex;
pub use private_extract_icon_ex::{PrivateExtractIconExA, PrivateExtractIconExW};

mod post_thread_message;
pub use post_thread_message::{PostThreadMessageA, PostThreadMessageW};

mod open_window_station;
pub use open_window_station::{OpenWindowStationA, OpenWindowStationW};

mod open_desktop;
pub use open_desktop::{OpenDesktopA, OpenDesktopW};

mod oem_to_char_buff;
pub use oem_to_char_buff::{OemToCharBuffA, OemToCharBuffW};

mod oem_to_char;
pub use oem_to_char::{OemToCharA, OemToCharW};

mod modify_menu;
pub use modify_menu::{ModifyMenuA, ModifyMenuW};

mod message_box_timeout;
pub use message_box_timeout::{MessageBoxTimeoutA, MessageBoxTimeoutW};

mod message_box_indirect;
pub use message_box_indirect::{MessageBoxIndirectA, MessageBoxIndirectW};

mod message_box_ex;
pub use message_box_ex::{MessageBoxExA, MessageBoxExW};

mod menu_window_proc;
pub use menu_window_proc::{MenuWindowProcA, MenuWindowProcW};

mod load_menu_indirect;
pub use load_menu_indirect::{LoadMenuIndirectA, LoadMenuIndirectW};

mod load_menu;
pub use load_menu::{LoadMenuA, LoadMenuW};

mod load_keyboard_layout;
pub use load_keyboard_layout::{LoadKeyboardLayoutA, LoadKeyboardLayoutW};

mod load_image;
pub use load_image::{LoadImageA, LoadImageW};

mod load_cursor_from_file;
pub use load_cursor_from_file::{LoadCursorFromFileA, LoadCursorFromFileW};

mod load_bitmap;
pub use load_bitmap::{LoadBitmapA, LoadBitmapW};

mod load_accelerators;
pub use load_accelerators::{LoadAcceleratorsA, LoadAcceleratorsW};

mod insert_menu_item;
pub use insert_menu_item::{InsertMenuItemA, InsertMenuItemW};

mod insert_menu;
pub use insert_menu::{InsertMenuA, InsertMenuW};

mod gray_string;
pub use gray_string::{GrayStringA, GrayStringW};

mod get_window_text_length;
pub use get_window_text_length::{GetWindowTextLengthA, GetWindowTextLengthW};

mod get_tabbed_text_extent;
pub use get_tabbed_text_extent::{GetTabbedTextExtentA, GetTabbedTextExtentW};

mod get_raw_input_device_info;
pub use get_raw_input_device_info::{GetRawInputDeviceInfoA, GetRawInputDeviceInfoW};

mod get_prop;
pub use get_prop::{GetPropA, GetPropW};

mod get_monitor_info;
pub use get_monitor_info::{GetMonitorInfoA, GetMonitorInfoW};

mod get_menu_string;
pub use get_menu_string::{GetMenuStringA, GetMenuStringW};

mod get_menu_item_info;
pub use get_menu_item_info::{GetMenuItemInfoA, GetMenuItemInfoW};

mod get_keyboard_layout_name;
pub use get_keyboard_layout_name::{GetKeyboardLayoutNameA, GetKeyboardLayoutNameW};

mod get_key_name_text;
pub use get_key_name_text::{GetKeyNameTextA, GetKeyNameTextW};

mod get_icon_info_ex;
pub use get_icon_info_ex::{GetIconInfoExA, GetIconInfoExW};

mod get_dlg_item_text;
pub use get_dlg_item_text::{GetDlgItemTextA, GetDlgItemTextW};

mod get_clipboard_format_name;
pub use get_clipboard_format_name::{GetClipboardFormatNameA, GetClipboardFormatNameW};

mod get_class_name;
pub use get_class_name::{GetClassNameA, GetClassNameW};

mod get_class_long_ptr;
pub use get_class_long_ptr::{GetClassLongPtrA, GetClassLongPtrW};

mod get_class_long;
pub use get_class_long::{GetClassLongA, GetClassLongW};

mod get_class_info;
pub use get_class_info::{GetClassInfoA, GetClassInfoW};

mod find_window_ex;
pub use find_window_ex::{FindWindowExA, FindWindowExW};

mod find_window;
pub use find_window::{FindWindowA, FindWindowW};

mod enum_window_stations;
pub use enum_window_stations::{EnumWindowStationsA, EnumWindowStationsW};

mod enum_props_ex;
pub use enum_props_ex::{EnumPropsExA, EnumPropsExW};

mod enum_props;
pub use enum_props::{EnumPropsA, EnumPropsW};

mod enum_display_settings_ex;
pub use enum_display_settings_ex::{EnumDisplaySettingsExA, EnumDisplaySettingsExW};

mod enum_display_settings;
pub use enum_display_settings::{EnumDisplaySettingsA, EnumDisplaySettingsW};

mod enum_display_devices;
pub use enum_display_devices::{EnumDisplayDevicesA, EnumDisplayDevicesW};

mod enum_desktops;
pub use enum_desktops::{EnumDesktopsA, EnumDesktopsW};

mod draw_text_ex;
pub use draw_text_ex::{DrawTextExA, DrawTextExW};

mod draw_state;
pub use draw_state::{DrawStateA, DrawStateW};

mod draw_caption_temp;
pub use draw_caption_temp::{DrawCaptionTempA, DrawCaptionTempW};

mod dlg_dir_select_ex;
pub use dlg_dir_select_ex::{DlgDirSelectExA, DlgDirSelectExW};

mod dlg_dir_select_combo_box_ex;
pub use dlg_dir_select_combo_box_ex::{DlgDirSelectComboBoxExA, DlgDirSelectComboBoxExW};

mod dlg_dir_list_combo_box;
pub use dlg_dir_list_combo_box::{DlgDirListComboBoxA, DlgDirListComboBoxW};

mod dlg_dir_list;
pub use dlg_dir_list::{DlgDirListA, DlgDirListW};

mod dialog_box_param;
pub use dialog_box_param::{DialogBoxParamA, DialogBoxParamW};

mod dialog_box_indirect_param;
pub use dialog_box_indirect_param::{DialogBoxIndirectParamA, DialogBoxIndirectParamW};

mod def_mdi_child_proc;
pub use def_mdi_child_proc::{DefMDIChildProcA, DefMDIChildProcW};

mod def_frame_proc;
pub use def_frame_proc::{DefFrameProcA, DefFrameProcW};

mod def_dlg_proc;
pub use def_dlg_proc::{DefDlgProcA, DefDlgProcW};

mod dde_query_string;
pub use dde_query_string::{DdeQueryStringA, DdeQueryStringW};

mod dde_initialize;
pub use dde_initialize::{DdeInitializeA, DdeInitializeW};

mod dde_create_string_handle;
pub use dde_create_string_handle::{DdeCreateStringHandleA, DdeCreateStringHandleW};

mod create_window_station;
pub use create_window_station::{CreateWindowStationA, CreateWindowStationW};

mod create_mdi_window;
pub use create_mdi_window::{CreateMDIWindowA, CreateMDIWindowW};

mod create_dialog_param;
pub use create_dialog_param::{CreateDialogParamA, CreateDialogParamW};

mod create_dialog_indirect_param;
pub use create_dialog_indirect_param::{CreateDialogIndirectParamA, CreateDialogIndirectParamW};

mod create_desktop_ex;
pub use create_desktop_ex::{CreateDesktopExA, CreateDesktopExW};

mod create_desktop;
pub use create_desktop::{CreateDesktopA, CreateDesktopW};

mod create_accelerator_table;
pub use create_accelerator_table::{CreateAcceleratorTableA, CreateAcceleratorTableW};

mod copy_accelerator_table;
pub use copy_accelerator_table::{CopyAcceleratorTableA, CopyAcceleratorTableW};

mod char_upper;
pub use char_upper::{CharUpperA, CharUpperW};

mod char_to_oem_buff;
pub use char_to_oem_buff::{CharToOemBuffA, CharToOemBuffW};

mod char_to_oem;
pub use char_to_oem::{CharToOemA, CharToOemW};

mod char_lower;
pub use char_lower::{CharLowerA, CharLowerW};

mod change_menu;
pub use change_menu::{ChangeMenuA, ChangeMenuW};

mod change_display_settings_ex;
pub use change_display_settings_ex::{ChangeDisplaySettingsExA, ChangeDisplaySettingsExW};

mod change_display_settings;
pub use change_display_settings::{ChangeDisplaySettingsA, ChangeDisplaySettingsW};

mod call_window_proc;
pub use call_window_proc::{CallWindowProcA, CallWindowProcW};

mod broadcast_system_message_ex;
pub use broadcast_system_message_ex::{BroadcastSystemMessageExA, BroadcastSystemMessageExW};

mod append_menu;
pub use append_menu::{AppendMenuA, AppendMenuW};

mod activate_keyboard_layout;
mod clipboard;
mod dde;
mod dialogs;
mod graphics;
mod input;
mod menus;
mod messages;
mod strings_format;
mod system;
mod windowing;
pub use activate_keyboard_layout::ActivateKeyboardLayout;
mod add_clipboard_format_listener;
pub use add_clipboard_format_listener::AddClipboardFormatListener;
mod add_visual_identifier;
pub use add_visual_identifier::AddVisualIdentifier;
mod adjust_window_rect_ex_for_dpi;
pub use adjust_window_rect_ex_for_dpi::AdjustWindowRectExForDpi;
mod align_rects;
pub use align_rects::AlignRects;
mod allow_foreground_activation;
pub use allow_foreground_activation::AllowForegroundActivation;
mod allow_set_foreground_window;
pub use allow_set_foreground_window::AllowSetForegroundWindow;
mod animate_window;
pub use animate_window::AnimateWindow;
mod any_popup;
pub use any_popup::AnyPopup;
mod apply_window_action;
pub use apply_window_action::ApplyWindowAction;
mod are_dpi_awareness_contexts_equal;
pub use are_dpi_awareness_contexts_equal::AreDpiAwarenessContextsEqual;
mod arrange_iconic_windows;
pub use arrange_iconic_windows::ArrangeIconicWindows;
mod attach_thread_input;
pub use attach_thread_input::AttachThreadInput;
mod begin_defer_window_pos;
pub use begin_defer_window_pos::BeginDeferWindowPos;
mod block_input;
pub use block_input::BlockInput;
mod bring_window_to_top;
pub use bring_window_to_top::BringWindowToTop;
mod broadcast_system_message;
pub use broadcast_system_message::{
    BroadcastSystemMessage, BroadcastSystemMessageA, BroadcastSystemMessageW,
};

mod build_reason_array;
pub use build_reason_array::BuildReasonArray;
mod calc_menu_bar;
pub use calc_menu_bar::CalcMenuBar;
mod calculate_popup_window_position;
pub use calculate_popup_window_position::CalculatePopupWindowPosition;
mod call_msg_filter;
pub use call_msg_filter::{CallMsgFilter, CallMsgFilterA, CallMsgFilterW};

mod call_next_hook_ex;
pub use call_next_hook_ex::CallNextHookEx;
mod cancel_shutdown;
pub use cancel_shutdown::CancelShutdown;
mod cascade_child_windows;
pub use cascade_child_windows::CascadeChildWindows;
mod cascade_windows;
pub use cascade_windows::CascadeWindows;
mod change_clipboard_chain;
pub use change_clipboard_chain::ChangeClipboardChain;
mod change_window_message_filter;
pub use change_window_message_filter::ChangeWindowMessageFilter;
mod change_window_message_filter_ex;
pub use change_window_message_filter_ex::ChangeWindowMessageFilterEx;
mod check_banned_one_core_transform;
pub use check_banned_one_core_transform::CheckBannedOneCoreTransformApi;
mod check_dbcs_enabled_ext;
pub use check_dbcs_enabled_ext::CheckDBCSEnabledExt;
mod check_dlg_button;
pub use check_dlg_button::CheckDlgButton;
mod check_menu_item;
pub use check_menu_item::CheckMenuItem;
mod check_menu_radio_item;
pub use check_menu_radio_item::CheckMenuRadioItem;
mod check_process_for_clipboard_access;
pub use check_process_for_clipboard_access::CheckProcessForClipboardAccess;
mod check_process_session;
pub use check_process_session::CheckProcessSession;
mod check_radio_button;
pub use check_radio_button::CheckRadioButton;
mod check_window_thread_desktop;
pub use check_window_thread_desktop::CheckWindowThreadDesktop;
mod child_window_from_point;
pub use child_window_from_point::ChildWindowFromPoint;
mod child_window_from_point_ex;
pub use child_window_from_point_ex::ChildWindowFromPointEx;
mod cli_imm_set_hot_key;
pub use cli_imm_set_hot_key::CliImmSetHotKey;
mod client_thread_setup;
pub use client_thread_setup::ClientThreadSetup;
mod client_to_screen;
pub use client_to_screen::ClientToScreen;
mod clip_cursor;
pub use clip_cursor::ClipCursor;
mod close_clipboard;
pub use close_clipboard::CloseClipboard;
mod close_desktop;
pub use close_desktop::CloseDesktop;
mod close_gesture_info_handle;
pub use close_gesture_info_handle::CloseGestureInfoHandle;
mod close_touch_input_handle;
pub use close_touch_input_handle::CloseTouchInputHandle;
mod close_window;
pub use close_window::CloseWindow;
mod close_window_station;
pub use close_window_station::CloseWindowStation;
mod console_control;
pub use console_control::ConsoleControl;
mod control_magnification;
pub use control_magnification::ControlMagnification;
mod convert_primary_pointer_to_mouse_drag;
pub use convert_primary_pointer_to_mouse_drag::ConvertPrimaryPointerToMouseDrag;
mod convert_to_intercept_window;
pub use convert_to_intercept_window::ConvertToInterceptWindow;
mod copy_icon;
pub use copy_icon::CopyIcon;
mod copy_image;
pub use copy_image::CopyImage;
mod copy_rect;
pub use copy_rect::CopyRect;
mod count_clipboard_formats;
pub use count_clipboard_formats::CountClipboardFormats;
mod create_caret;
pub use create_caret::CreateCaret;
mod create_cursor;
pub use create_cursor::CreateCursor;
mod create_d_composition_hwnd_target;
pub use create_d_composition_hwnd_target::CreateDCompositionHwndTarget;
mod create_dialog_indirect_param_aor_w;
pub use create_dialog_indirect_param_aor_w::CreateDialogIndirectParamAorW;
mod create_icon;
pub use create_icon::CreateIcon;
mod create_icon_from_resource;
pub use create_icon_from_resource::CreateIconFromResource;
mod create_icon_from_resource_ex;
pub use create_icon_from_resource_ex::CreateIconFromResourceEx;
mod create_icon_indirect;
pub use create_icon_indirect::CreateIconIndirect;
mod create_menu;
pub use create_menu::CreateMenu;
mod create_popup_menu;
pub use create_popup_menu::CreatePopupMenu;
mod create_synthetic_pointer_device;
pub use create_synthetic_pointer_device::CreateSyntheticPointerDevice;
mod create_synthetic_pointer_device2;
pub use create_synthetic_pointer_device2::CreateSyntheticPointerDevice2;
mod create_system_threads;
pub use create_system_threads::CreateSystemThreads;
mod create_window_in_band;
pub use create_window_in_band::CreateWindowInBand;
mod create_window_in_band_ex;
pub use create_window_in_band_ex::CreateWindowInBandEx;
mod create_window_indirect;
pub use create_window_indirect::CreateWindowIndirect;
mod csr_broadcast_system_message_ex_w;
pub use csr_broadcast_system_message_ex_w::CsrBroadcastSystemMessageExW;
mod ctx_init_user32;
pub use ctx_init_user32::CtxInitUser32;
mod dde_abandon_transaction;
pub use dde_abandon_transaction::DdeAbandonTransaction;
mod dde_access_data;
pub use dde_access_data::DdeAccessData;
mod dde_add_data;
pub use dde_add_data::DdeAddData;
mod dde_client_transaction;
pub use dde_client_transaction::DdeClientTransaction;
mod dde_cmp_string_handles;
pub use dde_cmp_string_handles::DdeCmpStringHandles;
mod dde_connect;
pub use dde_connect::DdeConnect;
mod dde_connect_list;
pub use dde_connect_list::DdeConnectList;
mod dde_create_data_handle;
pub use dde_create_data_handle::DdeCreateDataHandle;
mod dde_disconnect;
pub use dde_disconnect::DdeDisconnect;
mod dde_disconnect_list;
pub use dde_disconnect_list::DdeDisconnectList;
mod dde_enable_callback;
pub use dde_enable_callback::DdeEnableCallback;
mod dde_free_data_handle;
pub use dde_free_data_handle::DdeFreeDataHandle;
mod dde_free_string_handle;
pub use dde_free_string_handle::DdeFreeStringHandle;
mod dde_get_data;
pub use dde_get_data::DdeGetData;
mod dde_get_last_error;
pub use dde_get_last_error::DdeGetLastError;
mod dde_get_quality_of_service;
pub use dde_get_quality_of_service::DdeGetQualityOfService;
mod dde_impersonate_client;
pub use dde_impersonate_client::DdeImpersonateClient;
mod dde_keep_string_handle;
pub use dde_keep_string_handle::DdeKeepStringHandle;
mod dde_name_service;
pub use dde_name_service::DdeNameService;
mod dde_post_advise;
pub use dde_post_advise::DdePostAdvise;
mod dde_query_conv_info;
pub use dde_query_conv_info::DdeQueryConvInfo;
mod dde_query_next_server;
pub use dde_query_next_server::DdeQueryNextServer;
mod dde_reconnect;
pub use dde_reconnect::DdeReconnect;
mod dde_set_quality_of_service;
pub use dde_set_quality_of_service::DdeSetQualityOfService;
mod dde_set_user_handle;
pub use dde_set_user_handle::DdeSetUserHandle;
mod dde_unaccess_data;
pub use dde_unaccess_data::DdeUnaccessData;
mod dde_uninitialize;
pub use dde_uninitialize::DdeUninitialize;
mod def_raw_input_proc;
pub use def_raw_input_proc::DefRawInputProc;
mod defer_window_pos;
pub use defer_window_pos::DeferWindowPos;
mod defer_window_pos_and_band;
pub use defer_window_pos_and_band::DeferWindowPosAndBand;
mod delegate_input;
pub use delegate_input::DelegateInput;
mod delete_menu;
pub use delete_menu::DeleteMenu;
mod deregister_shell_hook_window;
pub use deregister_shell_hook_window::DeregisterShellHookWindow;
mod destroy_accelerator_table;
pub use destroy_accelerator_table::DestroyAcceleratorTable;
mod destroy_caret;
pub use destroy_caret::DestroyCaret;
mod destroy_cursor;
pub use destroy_cursor::DestroyCursor;
mod destroy_d_composition_hwnd_target;
pub use destroy_d_composition_hwnd_target::DestroyDCompositionHwndTarget;
mod destroy_icon;
pub use destroy_icon::DestroyIcon;
mod destroy_menu;
pub use destroy_menu::DestroyMenu;
mod destroy_reasons;
pub use destroy_reasons::DestroyReasons;
mod destroy_synthetic_pointer_device;
pub use destroy_synthetic_pointer_device::DestroySyntheticPointerDevice;
mod dialog_box_indirect_param_aor_w;
pub use dialog_box_indirect_param_aor_w::DialogBoxIndirectParamAorW;
mod disable_process_windows_ghosting;
pub use disable_process_windows_ghosting::DisableProcessWindowsGhosting;
mod display_config_get_device_info;
pub use display_config_get_device_info::DisplayConfigGetDeviceInfo;
mod display_config_set_device_info;
pub use display_config_set_device_info::DisplayConfigSetDeviceInfo;
mod display_exit_windows_warnings;
pub use display_exit_windows_warnings::DisplayExitWindowsWarnings;
mod do_sound_connect;
pub use do_sound_connect::DoSoundConnect;
mod do_sound_disconnect;
pub use do_sound_disconnect::DoSoundDisconnect;
mod drag_detect;
pub use drag_detect::DragDetect;
mod drag_object;
pub use drag_object::DragObject;
mod draw_animated_rects;
pub use draw_animated_rects::DrawAnimatedRects;
mod draw_caption;
pub use draw_caption::DrawCaption;
mod draw_edge;
pub use draw_edge::DrawEdge;
mod draw_focus_rect;
pub use draw_focus_rect::DrawFocusRect;
mod draw_frame;
pub use draw_frame::DrawFrame;
mod draw_frame_control;
pub use draw_frame_control::DrawFrameControl;
mod draw_icon;
pub use draw_icon::DrawIcon;
mod draw_icon_ex;
pub use draw_icon_ex::DrawIconEx;
mod draw_menu_bar;
pub use draw_menu_bar::DrawMenuBar;
mod draw_menu_bar_temp;
pub use draw_menu_bar_temp::DrawMenuBarTemp;
mod dwm_get_dx_rgn;
pub use dwm_get_dx_rgn::DwmGetDxRgn;
mod dwm_get_dx_shared_surface;
pub use dwm_get_dx_shared_surface::DwmGetDxSharedSurface;
mod dwm_get_remote_session_occlusion_event;
pub use dwm_get_remote_session_occlusion_event::DwmGetRemoteSessionOcclusionEvent;
mod dwm_get_remote_session_occlusion_state;
pub use dwm_get_remote_session_occlusion_state::DwmGetRemoteSessionOcclusionState;
mod dwm_kernel_shutdown;
pub use dwm_kernel_shutdown::DwmKernelShutdown;
mod dwm_kernel_startup;
pub use dwm_kernel_startup::DwmKernelStartup;
mod dwm_lock_screen_updates;
pub use dwm_lock_screen_updates::DwmLockScreenUpdates;
mod dwm_validate_window;
pub use dwm_validate_window::DwmValidateWindow;
mod dwm_window_notifications_enabled;
pub use dwm_window_notifications_enabled::DwmWindowNotificationsEnabled;
mod edit_wnd_proc;
pub use edit_wnd_proc::EditWndProc;
mod empty_clipboard;
pub use empty_clipboard::EmptyClipboard;
mod enable_menu_item;
pub use enable_menu_item::EnableMenuItem;
mod enable_mouse_in_pointer;
pub use enable_mouse_in_pointer::EnableMouseInPointer;
mod enable_non_client_dpi_scaling;
pub use enable_non_client_dpi_scaling::EnableNonClientDpiScaling;
mod enable_one_core_transform_mode;
pub use enable_one_core_transform_mode::EnableOneCoreTransformMode;
mod enable_scroll_bar;
pub use enable_scroll_bar::EnableScrollBar;
mod enable_session_for_mmcss;
pub use enable_session_for_mmcss::EnableSessionForMMCSS;
mod enable_window_shell_window_management_behavior;
pub use enable_window_shell_window_management_behavior::EnableWindowShellWindowManagementBehavior;
mod end_defer_window_pos;
pub use end_defer_window_pos::EndDeferWindowPos;
mod end_defer_window_pos_ex;
pub use end_defer_window_pos_ex::EndDeferWindowPosEx;
mod end_dialog;
pub use end_dialog::EndDialog;
mod end_menu;
pub use end_menu::EndMenu;
mod end_task;
pub use end_task::EndTask;
mod enter_move_size_loop;
pub use enter_move_size_loop::EnterMoveSizeLoop;
mod enter_reader_mode_helper;
pub use enter_reader_mode_helper::EnterReaderModeHelper;
mod enum_child_windows;
pub use enum_child_windows::EnumChildWindows;
mod enum_clipboard_formats;
pub use enum_clipboard_formats::EnumClipboardFormats;
mod enum_desktop_windows;
pub use enum_desktop_windows::EnumDesktopWindows;
mod enum_display_monitors;
pub use enum_display_monitors::EnumDisplayMonitors;
mod enum_thread_windows;
pub use enum_thread_windows::EnumThreadWindows;
mod equal_rect;
pub use equal_rect::EqualRect;
mod evaluate_proximity_to_polygon;
pub use evaluate_proximity_to_polygon::EvaluateProximityToPolygon;
mod evaluate_proximity_to_rect;
pub use evaluate_proximity_to_rect::EvaluateProximityToRect;
mod exclude_update_rgn;
pub use exclude_update_rgn::ExcludeUpdateRgn;
mod exit_windows_ex;
pub use exit_windows_ex::ExitWindowsEx;
mod flash_window;
pub use flash_window::FlashWindow;
mod flash_window_ex;
pub use flash_window_ex::FlashWindowEx;
mod frame_rect;
pub use frame_rect::FrameRect;
mod free_dd_el_param;
pub use free_dd_el_param::FreeDDElParam;
mod frost_crashed_window;
pub use frost_crashed_window::FrostCrashedWindow;
mod get_active_window;
pub use get_active_window::GetActiveWindow;
mod get_alt_tab_info;
pub use get_alt_tab_info::{GetAltTabInfo, GetAltTabInfoA, GetAltTabInfoW};

mod get_ancestor;
pub use get_ancestor::GetAncestor;
mod get_app_compat_flags;
pub use get_app_compat_flags::GetAppCompatFlags;
mod get_app_compat_flags2;
pub use get_app_compat_flags2::GetAppCompatFlags2;
mod get_auto_rotation_state;
pub use get_auto_rotation_state::GetAutoRotationState;
mod get_awareness_from_dpi_awareness_context;
pub use get_awareness_from_dpi_awareness_context::GetAwarenessFromDpiAwarenessContext;
mod get_cimssm;
pub use get_cimssm::GetCIMSSM;
mod get_capture;
pub use get_capture::GetCapture;
mod get_caret_blink_time;
pub use get_caret_blink_time::GetCaretBlinkTime;
mod get_caret_pos;
pub use get_caret_pos::GetCaretPos;
mod get_class_word;
pub use get_class_word::GetClassWord;
mod get_clip_cursor;
pub use get_clip_cursor::GetClipCursor;
mod get_clipboard_access_token;
pub use get_clipboard_access_token::GetClipboardAccessToken;
mod get_clipboard_data;
pub use get_clipboard_data::GetClipboardData;
mod get_clipboard_metadata;
pub use get_clipboard_metadata::GetClipboardMetadata;
mod get_clipboard_owner;
pub use get_clipboard_owner::GetClipboardOwner;
mod get_clipboard_sequence_number;
pub use get_clipboard_sequence_number::GetClipboardSequenceNumber;
mod get_clipboard_viewer;
pub use get_clipboard_viewer::GetClipboardViewer;
mod get_combo_box_info;
pub use get_combo_box_info::GetComboBoxInfo;
mod get_current_input_message_source;
pub use get_current_input_message_source::GetCurrentInputMessageSource;
mod get_current_monitor_topology_id;
pub use get_current_monitor_topology_id::GetCurrentMonitorTopologyId;
mod get_cursor;
pub use get_cursor::GetCursor;
mod get_cursor_frame_info;
pub use get_cursor_frame_info::GetCursorFrameInfo;
mod get_cursor_info;
pub use get_cursor_info::GetCursorInfo;
mod get_dc_ex;
pub use get_dc_ex::GetDCEx;
mod get_d_composition_hwnd_bitmap;
pub use get_d_composition_hwnd_bitmap::GetDCompositionHwndBitmap;
mod get_desktop_id;
pub use get_desktop_id::GetDesktopID;
mod get_dialog_base_units;
pub use get_dialog_base_units::GetDialogBaseUnits;
mod get_dialog_control_dpi_change_behavior;
pub use get_dialog_control_dpi_change_behavior::GetDialogControlDpiChangeBehavior;
mod get_dialog_dpi_change_behavior;
pub use get_dialog_dpi_change_behavior::GetDialogDpiChangeBehavior;
mod get_display_auto_rotation_preferences;
pub use get_display_auto_rotation_preferences::GetDisplayAutoRotationPreferences;
mod get_display_config_buffer_sizes;
pub use get_display_config_buffer_sizes::GetDisplayConfigBufferSizes;
mod get_dlg_ctrl_id;
pub use get_dlg_ctrl_id::GetDlgCtrlID;
mod get_dlg_item;
pub use get_dlg_item::GetDlgItem;
mod get_dlg_item_int;
pub use get_dlg_item_int::GetDlgItemInt;
mod get_double_click_time;
pub use get_double_click_time::GetDoubleClickTime;
mod get_dpi_awareness_context_for_process;
pub use get_dpi_awareness_context_for_process::GetDpiAwarenessContextForProcess;
mod get_dpi_for_monitor_internal;
pub use get_dpi_for_monitor_internal::GetDpiForMonitorInternal;
mod get_dpi_for_system;
pub use get_dpi_for_system::GetDpiForSystem;
mod get_dpi_for_window;
pub use get_dpi_for_window::GetDpiForWindow;
mod get_dpi_from_dpi_awareness_context;
pub use get_dpi_from_dpi_awareness_context::GetDpiFromDpiAwarenessContext;
mod get_extended_pointer_device_property;
pub use get_extended_pointer_device_property::GetExtendedPointerDeviceProperty;
mod get_gui_thread_info;
pub use get_gui_thread_info::GetGUIThreadInfo;
mod get_gesture_config;
pub use get_gesture_config::GetGestureConfig;
mod get_gesture_extra_args;
pub use get_gesture_extra_args::GetGestureExtraArgs;
mod get_gesture_info;
pub use get_gesture_info::GetGestureInfo;
mod get_gui_resources;
pub use get_gui_resources::GetGuiResources;
mod get_icon_info;
pub use get_icon_info::GetIconInfo;
mod get_input_desktop;
pub use get_input_desktop::GetInputDesktop;
mod get_input_locale_info;
pub use get_input_locale_info::GetInputLocaleInfo;
mod get_input_state;
pub use get_input_state::GetInputState;
mod get_internal_window_pos;
pub use get_internal_window_pos::GetInternalWindowPos;
mod get_kb_code_page;
pub use get_kb_code_page::GetKBCodePage;
mod get_keyboard_layout_list;
pub use get_keyboard_layout_list::GetKeyboardLayoutList;
mod get_keyboard_state;
pub use get_keyboard_state::GetKeyboardState;
mod get_last_active_popup;
pub use get_last_active_popup::GetLastActivePopup;
mod get_last_input_info;
pub use get_last_input_info::GetLastInputInfo;
mod get_layered_window_attributes;
pub use get_layered_window_attributes::GetLayeredWindowAttributes;
mod get_list_box_info;
pub use get_list_box_info::GetListBoxInfo;
mod get_magnification_desktop_color_effect;
pub use get_magnification_desktop_color_effect::GetMagnificationDesktopColorEffect;
mod get_magnification_desktop_magnification;
pub use get_magnification_desktop_magnification::GetMagnificationDesktopMagnification;
mod get_magnification_desktop_sampling_mode;
pub use get_magnification_desktop_sampling_mode::GetMagnificationDesktopSamplingMode;
mod get_magnification_lens_ctx_information;
pub use get_magnification_lens_ctx_information::GetMagnificationLensCtxInformation;
mod get_menu;
pub use get_menu::GetMenu;
mod get_menu_bar_info;
pub use get_menu_bar_info::GetMenuBarInfo;
mod get_menu_check_mark_dimensions;
pub use get_menu_check_mark_dimensions::GetMenuCheckMarkDimensions;
mod get_menu_context_help_id;
pub use get_menu_context_help_id::GetMenuContextHelpId;
mod get_menu_default_item;
pub use get_menu_default_item::GetMenuDefaultItem;
mod get_menu_info;
pub use get_menu_info::GetMenuInfo;
mod get_menu_item_count;
pub use get_menu_item_count::GetMenuItemCount;
mod get_menu_item_id;
pub use get_menu_item_id::GetMenuItemID;
mod get_menu_item_rect;
pub use get_menu_item_rect::GetMenuItemRect;
mod get_menu_state;
pub use get_menu_state::GetMenuState;
mod get_message_extra_info;
pub use get_message_extra_info::GetMessageExtraInfo;
mod get_message_pos;
pub use get_message_pos::GetMessagePos;
mod get_message_time;
pub use get_message_time::GetMessageTime;
mod get_mouse_move_points_ex;
pub use get_mouse_move_points_ex::GetMouseMovePointsEx;
mod get_next_dlg_group_item;
pub use get_next_dlg_group_item::GetNextDlgGroupItem;
mod get_next_dlg_tab_item;
pub use get_next_dlg_tab_item::GetNextDlgTabItem;
mod get_open_clipboard_window;
pub use get_open_clipboard_window::GetOpenClipboardWindow;
mod get_parent;
pub use get_parent::GetParent;
mod get_pointer_cursor_id;
pub use get_pointer_cursor_id::GetPointerCursorId;
mod get_pointer_device;
pub use get_pointer_device::GetPointerDevice;
mod get_pointer_device_cursors;
pub use get_pointer_device_cursors::GetPointerDeviceCursors;
mod get_pointer_device_input_space;
pub use get_pointer_device_input_space::GetPointerDeviceInputSpace;
mod get_pointer_device_orientation;
pub use get_pointer_device_orientation::GetPointerDeviceOrientation;
mod get_pointer_device_properties;
pub use get_pointer_device_properties::GetPointerDeviceProperties;
mod get_pointer_device_rects;
pub use get_pointer_device_rects::GetPointerDeviceRects;
mod get_pointer_devices;
pub use get_pointer_devices::GetPointerDevices;
mod get_pointer_frame_arrival_times;
pub use get_pointer_frame_arrival_times::GetPointerFrameArrivalTimes;
mod get_pointer_frame_info;
pub use get_pointer_frame_info::GetPointerFrameInfo;
mod get_pointer_frame_info_history;
pub use get_pointer_frame_info_history::GetPointerFrameInfoHistory;
mod get_pointer_frame_pen_info;
pub use get_pointer_frame_pen_info::GetPointerFramePenInfo;
mod get_pointer_frame_pen_info_history;
pub use get_pointer_frame_pen_info_history::GetPointerFramePenInfoHistory;
mod get_pointer_frame_times;
pub use get_pointer_frame_times::GetPointerFrameTimes;
mod get_pointer_frame_touch_info;
pub use get_pointer_frame_touch_info::GetPointerFrameTouchInfo;
mod get_pointer_frame_touch_info_history;
pub use get_pointer_frame_touch_info_history::GetPointerFrameTouchInfoHistory;
mod get_pointer_frame_touchpad_info;
pub use get_pointer_frame_touchpad_info::GetPointerFrameTouchpadInfo;
mod get_pointer_frame_touchpad_info_history;
pub use get_pointer_frame_touchpad_info_history::GetPointerFrameTouchpadInfoHistory;
mod get_pointer_info;
pub use get_pointer_info::GetPointerInfo;
mod get_pointer_info_history;
pub use get_pointer_info_history::GetPointerInfoHistory;
mod get_pointer_input_transform;
pub use get_pointer_input_transform::GetPointerInputTransform;
mod get_pointer_pen_info;
pub use get_pointer_pen_info::GetPointerPenInfo;
mod get_pointer_pen_info_history;
pub use get_pointer_pen_info_history::GetPointerPenInfoHistory;
mod get_pointer_touch_info;
pub use get_pointer_touch_info::GetPointerTouchInfo;
mod get_pointer_touch_info_history;
pub use get_pointer_touch_info_history::GetPointerTouchInfoHistory;
mod get_pointer_touchpad_info;
pub use get_pointer_touchpad_info::GetPointerTouchpadInfo;
mod get_pointer_touchpad_info_history;
pub use get_pointer_touchpad_info_history::GetPointerTouchpadInfoHistory;
mod get_pointer_type;
pub use get_pointer_type::GetPointerType;
mod get_priority_clipboard_format;
pub use get_priority_clipboard_format::GetPriorityClipboardFormat;
mod get_process_default_layout;
pub use get_process_default_layout::GetProcessDefaultLayout;
mod get_process_dpi_awareness_internal;
pub use get_process_dpi_awareness_internal::GetProcessDpiAwarenessInternal;
mod get_process_ui_context_information;
pub use get_process_ui_context_information::GetProcessUIContextInformation;
mod get_progman_window;
pub use get_progman_window::GetProgmanWindow;
mod get_queue_status;
pub use get_queue_status::GetQueueStatus;
mod get_raw_input_buffer;
pub use get_raw_input_buffer::GetRawInputBuffer;
mod get_raw_input_data;
pub use get_raw_input_data::GetRawInputData;
mod get_raw_input_device_list;
pub use get_raw_input_device_list::GetRawInputDeviceList;
mod get_raw_pointer_device_data;
pub use get_raw_pointer_device_data::GetRawPointerDeviceData;
mod get_reason_title_from_reason_code;
pub use get_reason_title_from_reason_code::GetReasonTitleFromReasonCode;
mod get_registered_raw_input_devices;
pub use get_registered_raw_input_devices::GetRegisteredRawInputDevices;
mod get_scroll_bar_info;
pub use get_scroll_bar_info::GetScrollBarInfo;
mod get_scroll_info;
pub use get_scroll_info::GetScrollInfo;
mod get_scroll_pos;
pub use get_scroll_pos::GetScrollPos;
mod get_scroll_range;
pub use get_scroll_range::GetScrollRange;
mod get_send_message_receiver;
pub use get_send_message_receiver::GetSendMessageReceiver;
mod get_shell_change_notify_window;
pub use get_shell_change_notify_window::GetShellChangeNotifyWindow;
mod get_shell_window;
pub use get_shell_window::GetShellWindow;
mod get_sub_menu;
pub use get_sub_menu::GetSubMenu;
mod get_sys_color;
pub use get_sys_color::GetSysColor;
mod get_sys_color_brush;
pub use get_sys_color_brush::GetSysColorBrush;
mod get_system_dpi_for_process;
pub use get_system_dpi_for_process::GetSystemDpiForProcess;
mod get_system_menu;
pub use get_system_menu::GetSystemMenu;
mod get_system_metrics_for_dpi;
pub use get_system_metrics_for_dpi::GetSystemMetricsForDpi;
mod get_taskman_window;
pub use get_taskman_window::GetTaskmanWindow;
mod get_thread_desktop;
pub use get_thread_desktop::GetThreadDesktop;
mod get_thread_dpi_awareness_context;
pub use get_thread_dpi_awareness_context::GetThreadDpiAwarenessContext;
mod get_thread_dpi_hosting_behavior;
pub use get_thread_dpi_hosting_behavior::GetThreadDpiHostingBehavior;
mod get_title_bar_info;
pub use get_title_bar_info::GetTitleBarInfo;
mod get_top_level_window;
pub use get_top_level_window::GetTopLevelWindow;
mod get_top_window;
pub use get_top_window::GetTopWindow;
mod get_touch_input_info;
pub use get_touch_input_info::GetTouchInputInfo;
mod get_unpredicted_message_pos;
pub use get_unpredicted_message_pos::GetUnpredictedMessagePos;
mod get_update_rect;
pub use get_update_rect::GetUpdateRect;
mod get_update_rgn;
pub use get_update_rgn::GetUpdateRgn;
mod get_updated_clipboard_formats;
pub use get_updated_clipboard_formats::GetUpdatedClipboardFormats;
mod get_user_object_information;
pub use get_user_object_information::GetUserObjectInformationA;
mod get_user_object_security;
pub use get_user_object_security::GetUserObjectSecurity;
mod get_win_station_info;
pub use get_win_station_info::GetWinStationInfo;
mod get_window;
pub use get_window::GetWindow;
mod get_window_band;
pub use get_window_band::GetWindowBand;
mod get_window_composition_attribute;
pub use get_window_composition_attribute::GetWindowCompositionAttribute;
mod get_window_composition_info;
pub use get_window_composition_info::GetWindowCompositionInfo;
mod get_window_context_help_id;
pub use get_window_context_help_id::GetWindowContextHelpId;
mod get_window_dc;
pub use get_window_dc::GetWindowDC;
mod get_window_display_affinity;
pub use get_window_display_affinity::GetWindowDisplayAffinity;
mod get_window_dpi_awareness_context;
pub use get_window_dpi_awareness_context::GetWindowDpiAwarenessContext;
mod get_window_dpi_hosting_behavior;
pub use get_window_dpi_hosting_behavior::GetWindowDpiHostingBehavior;
mod get_window_feedback_setting;
pub use get_window_feedback_setting::GetWindowFeedbackSetting;
mod get_window_info;
pub use get_window_info::GetWindowInfo;
mod get_window_minimize_rect;
pub use get_window_minimize_rect::GetWindowMinimizeRect;
mod get_window_module_file_name;
pub use get_window_module_file_name::{
    GetWindowModuleFileName, GetWindowModuleFileNameA, GetWindowModuleFileNameW,
};

mod get_window_placement;
pub use get_window_placement::GetWindowPlacement;
mod get_window_process_handle;
pub use get_window_process_handle::GetWindowProcessHandle;
mod get_window_rgn;
pub use get_window_rgn::GetWindowRgn;
mod get_window_rgn_box;
pub use get_window_rgn_box::GetWindowRgnBox;
mod get_window_rgn_ex;
pub use get_window_rgn_ex::GetWindowRgnEx;
mod get_window_thread_process_id;
pub use get_window_thread_process_id::GetWindowThreadProcessId;
mod get_window_word;
pub use get_window_word::GetWindowWord;
mod ghost_window_from_hung_window;
pub use ghost_window_from_hung_window::GhostWindowFromHungWindow;
mod handle_delegated_input;
pub use handle_delegated_input::HandleDelegatedInput;
mod hide_caret;
pub use hide_caret::HideCaret;
mod hilite_menu_item;
pub use hilite_menu_item::HiliteMenuItem;
mod hung_window_from_ghost_window;
pub use hung_window_from_ghost_window::HungWindowFromGhostWindow;
mod imp_get_ime;
pub use imp_get_ime::{IMPGetIMEA, IMPGetIMEW};

mod imp_query_ime;
pub use imp_query_ime::{IMPQueryIMEA, IMPQueryIMEW};

mod imp_set_ime;
pub use imp_set_ime::{IMPSetIMEA, IMPSetIMEW};

mod impersonate_dde_client_window;
pub use impersonate_dde_client_window::ImpersonateDdeClientWindow;
mod in_send_message;
pub use in_send_message::InSendMessage;
mod in_send_message_ex;
pub use in_send_message_ex::InSendMessageEx;
mod inflate_rect;
pub use inflate_rect::InflateRect;
mod inherit_window_monitor;
pub use inherit_window_monitor::InheritWindowMonitor;
mod init_d_manip_hook;
pub use init_d_manip_hook::InitDManipHook;
mod initialize_generic_hid_injection;
pub use initialize_generic_hid_injection::InitializeGenericHidInjection;
mod initialize_input_device_injection;
pub use initialize_input_device_injection::InitializeInputDeviceInjection;
mod initialize_lpk_hooks;
pub use initialize_lpk_hooks::InitializeLpkHooks;
mod initialize_pointer_device_injection;
pub use initialize_pointer_device_injection::InitializePointerDeviceInjection;
mod initialize_pointer_device_injection_ex;
pub use initialize_pointer_device_injection_ex::InitializePointerDeviceInjectionEx;
mod initialize_touch_injection;
pub use initialize_touch_injection::InitializeTouchInjection;
mod inject_device_input;
pub use inject_device_input::InjectDeviceInput;
mod inject_generic_hid_input;
pub use inject_generic_hid_input::InjectGenericHidInput;
mod inject_keyboard_input;
pub use inject_keyboard_input::InjectKeyboardInput;
mod inject_mouse_input;
pub use inject_mouse_input::InjectMouseInput;
mod inject_pointer_input;
pub use inject_pointer_input::InjectPointerInput;
mod inject_synthetic_pointer_input;
pub use inject_synthetic_pointer_input::InjectSyntheticPointerInput;
mod inject_touch_input;
pub use inject_touch_input::InjectTouchInput;
mod input_space_region_from_point;
pub use input_space_region_from_point::InputSpaceRegionFromPoint;
mod internal_get_window_icon;
pub use internal_get_window_icon::InternalGetWindowIcon;
mod internal_get_window_text;
pub use internal_get_window_text::InternalGetWindowText;
mod intersect_rect;
pub use intersect_rect::IntersectRect;
mod invalidate_rgn;
pub use invalidate_rgn::InvalidateRgn;
mod invert_rect;
pub use invert_rect::InvertRect;
mod is_child;
pub use is_child::IsChild;
mod is_clipboard_format_available;
pub use is_clipboard_format_available::IsClipboardFormatAvailable;
mod is_dialog_message;
pub use is_dialog_message::{IsDialogMessage, IsDialogMessageA, IsDialogMessageW};

mod is_dlg_button_checked;
pub use is_dlg_button_checked::IsDlgButtonChecked;
mod is_gui_thread;
pub use is_gui_thread::IsGUIThread;
mod is_hung_app_window;
pub use is_hung_app_window::IsHungAppWindow;
mod is_iconic;
pub use is_iconic::IsIconic;
mod is_immersive_process;
pub use is_immersive_process::IsImmersiveProcess;
mod is_in_desktop_window_band;
pub use is_in_desktop_window_band::IsInDesktopWindowBand;
mod is_intercept_window;
pub use is_intercept_window::IsInterceptWindow;
mod is_menu;
pub use is_menu::IsMenu;
mod is_mouse_in_pointer_enabled;
pub use is_mouse_in_pointer_enabled::IsMouseInPointerEnabled;
mod is_one_core_transform_mode;
pub use is_one_core_transform_mode::IsOneCoreTransformMode;
mod is_process_dpi_aware;
pub use is_process_dpi_aware::IsProcessDPIAware;
mod is_rect_empty;
pub use is_rect_empty::IsRectEmpty;
mod is_set_enabled;
pub use is_set_enabled::IsSETEnabled;
mod is_server_side_window;
pub use is_server_side_window::IsServerSideWindow;
mod is_thread_desktop_composited;
pub use is_thread_desktop_composited::IsThreadDesktopComposited;
mod is_thread_message_queue_attached;
pub use is_thread_message_queue_attached::IsThreadMessageQueueAttached;
mod is_thread_tsf_event_aware;
pub use is_thread_tsf_event_aware::IsThreadTSFEventAware;
mod is_top_level_window;
pub use is_top_level_window::IsTopLevelWindow;
mod is_touch_window;
pub use is_touch_window::IsTouchWindow;
mod is_valid_dpi_awareness_context;
pub use is_valid_dpi_awareness_context::IsValidDpiAwarenessContext;
mod is_win_event_hook_installed;
pub use is_win_event_hook_installed::IsWinEventHookInstalled;
mod is_window_arranged;
pub use is_window_arranged::IsWindowArranged;
mod is_window_display_change_suppressed;
pub use is_window_display_change_suppressed::IsWindowDisplayChangeSuppressed;
mod is_window_enabled;
pub use is_window_enabled::IsWindowEnabled;
mod is_window_in_destroy;
pub use is_window_in_destroy::IsWindowInDestroy;
mod is_window_redirected_for_print;
pub use is_window_redirected_for_print::IsWindowRedirectedForPrint;
mod is_window_unicode;
pub use is_window_unicode::IsWindowUnicode;
mod is_wow64_message;
pub use is_wow64_message::IsWow64Message;
mod is_zoomed;
pub use is_zoomed::IsZoomed;
mod kill_timer;
pub use kill_timer::KillTimer;
pub use load_cursor::LoadCursorW;
pub use load_icon::LoadIconW;
mod load_keyboard_layout_ex;
pub use load_keyboard_layout_ex::LoadKeyboardLayoutEx;
mod load_local_fonts;
pub use load_local_fonts::LoadLocalFonts;
mod load_remote_fonts;
pub use load_remote_fonts::LoadRemoteFonts;
mod lock_set_foreground_window;
pub use lock_set_foreground_window::LockSetForegroundWindow;
mod lock_window_station;
pub use lock_window_station::LockWindowStation;
mod lock_window_update;
pub use lock_window_update::LockWindowUpdate;
mod lock_work_station;
pub use lock_work_station::LockWorkStation;
mod logical_to_physical_point;
pub use logical_to_physical_point::LogicalToPhysicalPoint;
mod logical_to_physical_point_for_per_monitor_dpi;
pub use logical_to_physical_point_for_per_monitor_dpi::LogicalToPhysicalPointForPerMonitorDPI;
mod lookup_icon_id_from_directory;
pub use lookup_icon_id_from_directory::LookupIconIdFromDirectory;
mod lookup_icon_id_from_directory_ex;
pub use lookup_icon_id_from_directory_ex::LookupIconIdFromDirectoryEx;
mod mb_to_wcs_ex;
pub use mb_to_wcs_ex::MBToWCSEx;
mod mb_to_wcs_ext;
pub use mb_to_wcs_ext::MBToWCSExt;
mod mb_get_string;
pub use mb_get_string::MB_GetString;
mod mit_get_cursor_update_handle;
pub use mit_get_cursor_update_handle::MITGetCursorUpdateHandle;
mod mit_set_last_input_recipient;
pub use mit_set_last_input_recipient::MITSetLastInputRecipient;
mod mit_synthesize_touch_input;
pub use mit_synthesize_touch_input::MITSynthesizeTouchInput;
mod make_thread_tsf_event_aware;
pub use make_thread_tsf_event_aware::MakeThreadTSFEventAware;
mod map_dialog_rect;
pub use map_dialog_rect::MapDialogRect;
mod map_points_by_visual_identifier;
pub use map_points_by_visual_identifier::MapPointsByVisualIdentifier;
mod map_virtual_key_ex;
pub use map_virtual_key_ex::{MapVirtualKeyExA, MapVirtualKeyExW};
mod map_visual_relative_points;
pub use map_visual_relative_points::MapVisualRelativePoints;
mod map_window_points;
pub use map_window_points::MapWindowPoints;
mod menu_item_from_point;
pub use menu_item_from_point::MenuItemFromPoint;
mod message_beep;
pub use message_beep::MessageBeep;
mod monitor_from_point;
pub use monitor_from_point::MonitorFromPoint;
mod monitor_from_rect;
pub use monitor_from_rect::MonitorFromRect;
mod monitor_from_window;
pub use monitor_from_window::MonitorFromWindow;
mod move_window;
pub use move_window::MoveWindow;
mod msg_wait_for_multiple_objects;
pub use msg_wait_for_multiple_objects::MsgWaitForMultipleObjects;
mod msg_wait_for_multiple_objects_ex;
pub use msg_wait_for_multiple_objects_ex::MsgWaitForMultipleObjectsEx;
mod notify_overlay_window;
pub use notify_overlay_window::NotifyOverlayWindow;
mod notify_win_event;
pub use notify_win_event::NotifyWinEvent;
mod oem_key_scan;
pub use oem_key_scan::OemKeyScan;
mod offset_rect;
pub use offset_rect::OffsetRect;
mod open_clipboard;
pub use open_clipboard::OpenClipboard;
mod open_icon;
pub use open_icon::OpenIcon;
mod open_input_desktop;
pub use open_input_desktop::OpenInputDesktop;
mod open_thread_desktop;
pub use open_thread_desktop::OpenThreadDesktop;
mod pack_dd_el_param;
pub use pack_dd_el_param::PackDDElParam;
mod pack_touch_hit_testing_proximity_evaluation;
pub use pack_touch_hit_testing_proximity_evaluation::PackTouchHitTestingProximityEvaluation;
mod paint_desktop;
pub use paint_desktop::PaintDesktop;
mod paint_menu_bar;
pub use paint_menu_bar::PaintMenuBar;
mod paint_monitor;
pub use paint_monitor::PaintMonitor;
mod physical_to_logical_point;
pub use physical_to_logical_point::PhysicalToLogicalPoint;
mod physical_to_logical_point_for_per_monitor_dpi;
pub use physical_to_logical_point_for_per_monitor_dpi::PhysicalToLogicalPointForPerMonitorDPI;
mod print_window;
pub use print_window::PrintWindow;
mod private_register_ics_proc;
pub use private_register_ics_proc::PrivateRegisterICSProc;
mod pt_in_rect;
pub use pt_in_rect::PtInRect;
mod query_bsdr_window;
pub use query_bsdr_window::QueryBSDRWindow;
mod query_display_config;
pub use query_display_config::QueryDisplayConfig;
mod query_send_message;
pub use query_send_message::QuerySendMessage;
mod rim_add_input_observer;
pub use rim_add_input_observer::RIMAddInputObserver;
mod rim_are_sibling_devices;
pub use rim_are_sibling_devices::RIMAreSiblingDevices;
mod rim_device_io_control;
pub use rim_device_io_control::RIMDeviceIoControl;
mod rim_enable_monitor_mapping_for_device;
pub use rim_enable_monitor_mapping_for_device::RIMEnableMonitorMappingForDevice;
mod rim_free_input_buffer;
pub use rim_free_input_buffer::RIMFreeInputBuffer;
mod rim_get_device_preparsed_data;
pub use rim_get_device_preparsed_data::RIMGetDevicePreparsedData;
mod rim_get_device_preparsed_data_lockfree;
pub use rim_get_device_preparsed_data_lockfree::RIMGetDevicePreparsedDataLockfree;
mod rim_get_device_properties;
pub use rim_get_device_properties::RIMGetDeviceProperties;
mod rim_get_device_properties_lockfree;
pub use rim_get_device_properties_lockfree::RIMGetDevicePropertiesLockfree;
mod rim_get_physical_device_rect;
pub use rim_get_physical_device_rect::RIMGetPhysicalDeviceRect;
mod rim_get_source_process_id;
pub use rim_get_source_process_id::RIMGetSourceProcessId;
mod rim_observe_next_input;
pub use rim_observe_next_input::RIMObserveNextInput;
mod rim_on_async_pnp_work_notification;
pub use rim_on_async_pnp_work_notification::RIMOnAsyncPnpWorkNotification;
mod rim_on_pnp_notification;
pub use rim_on_pnp_notification::RIMOnPnpNotification;
mod rim_on_timer_notification;
pub use rim_on_timer_notification::RIMOnTimerNotification;
mod rim_query_device_path;
pub use rim_query_device_path::RIMQueryDevicePath;
mod rim_read_input;
pub use rim_read_input::RIMReadInput;
mod rim_register_for_input;
pub use rim_register_for_input::RIMRegisterForInput;
mod rim_register_for_input_ex;
pub use rim_register_for_input_ex::RIMRegisterForInputEx;
mod rim_remove_input_observer;
pub use rim_remove_input_observer::RIMRemoveInputObserver;
mod rim_set_extended_device_property;
pub use rim_set_extended_device_property::RIMSetExtendedDeviceProperty;
mod rim_set_test_mode_status;
pub use rim_set_test_mode_status::RIMSetTestModeStatus;
mod rim_unregister_for_input;
pub use rim_unregister_for_input::RIMUnregisterForInput;
mod rim_update_input_observer_registration;
pub use rim_update_input_observer_registration::RIMUpdateInputObserverRegistration;
mod raise_lower_shell_window;
pub use raise_lower_shell_window::RaiseLowerShellWindow;
mod real_child_window_from_point;
pub use real_child_window_from_point::RealChildWindowFromPoint;
mod real_get_window_class;
pub use real_get_window_class::{RealGetWindowClass, RealGetWindowClassA, RealGetWindowClassW};

mod reason_code_needs_bug_id;
pub use reason_code_needs_bug_id::ReasonCodeNeedsBugID;
mod reason_code_needs_comment;
pub use reason_code_needs_comment::ReasonCodeNeedsComment;
mod record_shutdown_reason;
pub use record_shutdown_reason::RecordShutdownReason;
mod redraw_window;
pub use redraw_window::RedrawWindow;
mod register_bsdr_window;
pub use register_bsdr_window::RegisterBSDRWindow;
mod register_cloaked_notification;
pub use register_cloaked_notification::RegisterCloakedNotification;
mod register_d_manip_hook;
pub use register_d_manip_hook::RegisterDManipHook;
mod register_error_reporting_dialog;
pub use register_error_reporting_dialog::RegisterErrorReportingDialog;
mod register_for_custom_dock_targets;
pub use register_for_custom_dock_targets::RegisterForCustomDockTargets;
mod register_for_tooltip_dismiss_notification;
pub use register_for_tooltip_dismiss_notification::RegisterForTooltipDismissNotification;
mod register_frost_window;
pub use register_frost_window::RegisterFrostWindow;
mod register_ghost_window;
pub use register_ghost_window::RegisterGhostWindow;
mod register_hot_key;
pub use register_hot_key::RegisterHotKey;
mod register_logon_process;
pub use register_logon_process::RegisterLogonProcess;
mod register_message_pump_hook;
pub use register_message_pump_hook::RegisterMessagePumpHook;
mod register_pointer_device_notifications;
pub use register_pointer_device_notifications::RegisterPointerDeviceNotifications;
mod register_pointer_input_target;
pub use register_pointer_input_target::RegisterPointerInputTarget;
mod register_pointer_input_target_ex;
pub use register_pointer_input_target_ex::RegisterPointerInputTargetEx;
mod register_power_setting_notification;
pub use register_power_setting_notification::RegisterPowerSettingNotification;
mod register_raw_input_devices;
pub use register_raw_input_devices::RegisterRawInputDevices;
mod register_services_process;
pub use register_services_process::RegisterServicesProcess;
mod register_session_port;
pub use register_session_port::RegisterSessionPort;
mod register_shell_hook_window;
pub use register_shell_hook_window::RegisterShellHookWindow;
mod register_suspend_resume_notification;
pub use register_suspend_resume_notification::RegisterSuspendResumeNotification;
mod register_system_thread;
pub use register_system_thread::RegisterSystemThread;
mod register_tasklist;
pub use register_tasklist::RegisterTasklist;
mod register_touch_hit_testing_window;
pub use register_touch_hit_testing_window::RegisterTouchHitTestingWindow;
mod register_touch_window;
pub use register_touch_window::RegisterTouchWindow;
mod register_touchpad_capable_thread;
pub use register_touchpad_capable_thread::RegisterTouchpadCapableThread;
mod register_touchpad_capable_window;
pub use register_touchpad_capable_window::RegisterTouchpadCapableWindow;
mod register_user_api_hook;
pub use register_user_api_hook::RegisterUserApiHook;
mod release_capture;
pub use release_capture::ReleaseCapture;
mod release_dwm_hit_test_waiters;
pub use release_dwm_hit_test_waiters::ReleaseDwmHitTestWaiters;
mod remove_clipboard_format_listener;
pub use remove_clipboard_format_listener::RemoveClipboardFormatListener;
mod remove_injection_device;
pub use remove_injection_device::RemoveInjectionDevice;
mod remove_menu;
pub use remove_menu::RemoveMenu;
mod remove_thread_tsf_event_awareness;
pub use remove_thread_tsf_event_awareness::RemoveThreadTSFEventAwareness;
mod remove_visual_identifier;
pub use remove_visual_identifier::RemoveVisualIdentifier;
mod reply_message;
pub use reply_message::ReplyMessage;
mod report_inertia;
pub use report_inertia::ReportInertia;
mod resolve_desktop_for_wow;
pub use resolve_desktop_for_wow::ResolveDesktopForWOW;
mod reuse_dd_el_param;
pub use reuse_dd_el_param::ReuseDDElParam;
mod screen_to_client;
pub use screen_to_client::ScreenToClient;
mod scroll_children;
pub use scroll_children::ScrollChildren;
mod scroll_dc;
pub use scroll_dc::ScrollDC;
mod scroll_window;
pub use scroll_window::ScrollWindow;
mod scroll_window_ex;
pub use scroll_window_ex::ScrollWindowEx;
mod send_input;
pub use send_input::SendInput;
mod set_active_window;
pub use set_active_window::SetActiveWindow;
mod set_additional_foreground_boost_processes;
pub use set_additional_foreground_boost_processes::SetAdditionalForegroundBoostProcesses;
mod set_capture;
pub use set_capture::SetCapture;
mod set_caret_blink_time;
pub use set_caret_blink_time::SetCaretBlinkTime;
mod set_caret_pos;
pub use set_caret_pos::SetCaretPos;
mod set_class_word;
pub use set_class_word::SetClassWord;
mod set_clipboard_data;
pub use set_clipboard_data::SetClipboardData;
mod set_clipboard_viewer;
pub use set_clipboard_viewer::SetClipboardViewer;
mod set_coalescable_timer;
pub use set_coalescable_timer::SetCoalescableTimer;
mod set_core_window;
pub use set_core_window::SetCoreWindow;
mod set_covered_window_states;
pub use set_covered_window_states::SetCoveredWindowStates;
mod set_cursor_contents;
pub use set_cursor_contents::SetCursorContents;
mod set_debug_error_level;
pub use set_debug_error_level::SetDebugErrorLevel;
mod set_desk_wallpaper;
pub use set_desk_wallpaper::SetDeskWallpaper;
mod set_desktop_color_transform;
pub use set_desktop_color_transform::SetDesktopColorTransform;
mod set_dialog_control_dpi_change_behavior;
pub use set_dialog_control_dpi_change_behavior::SetDialogControlDpiChangeBehavior;
mod set_dialog_dpi_change_behavior;
pub use set_dialog_dpi_change_behavior::SetDialogDpiChangeBehavior;
mod set_display_auto_rotation_preferences;
pub use set_display_auto_rotation_preferences::SetDisplayAutoRotationPreferences;
mod set_display_config;
pub use set_display_config::SetDisplayConfig;
mod set_dlg_item_int;
pub use set_dlg_item_int::SetDlgItemInt;
mod set_double_click_time;
pub use set_double_click_time::SetDoubleClickTime;
mod set_feature_report_response;
pub use set_feature_report_response::SetFeatureReportResponse;
mod set_foreground_redirection_for_activation_object;
pub use set_foreground_redirection_for_activation_object::SetForegroundRedirectionForActivationObject;
mod set_fullscreen_magnifier_offsets_dwm_updated;
pub use set_fullscreen_magnifier_offsets_dwm_updated::SetFullscreenMagnifierOffsetsDWMUpdated;
mod set_gesture_config;
pub use set_gesture_config::SetGestureConfig;
mod set_internal_window_pos;
pub use set_internal_window_pos::SetInternalWindowPos;
mod set_keyboard_state;
pub use set_keyboard_state::SetKeyboardState;
mod set_last_error_ex;
pub use set_last_error_ex::SetLastErrorEx;
mod set_layered_window_attributes;
pub use set_layered_window_attributes::SetLayeredWindowAttributes;
mod set_magnification_desktop_color_effect;
pub use set_magnification_desktop_color_effect::SetMagnificationDesktopColorEffect;
mod set_magnification_desktop_magnification;
pub use set_magnification_desktop_magnification::SetMagnificationDesktopMagnification;
mod set_magnification_desktop_magnifier_offsets_dwm_updated;
pub use set_magnification_desktop_magnifier_offsets_dwm_updated::SetMagnificationDesktopMagnifierOffsetsDWMUpdated;
mod set_magnification_desktop_sampling_mode;
pub use set_magnification_desktop_sampling_mode::SetMagnificationDesktopSamplingMode;
mod set_magnification_lens_ctx_information;
pub use set_magnification_lens_ctx_information::SetMagnificationLensCtxInformation;
mod set_max_touchpad_sensitivity;
pub use set_max_touchpad_sensitivity::SetMaxTouchpadSensitivity;
mod set_menu;
pub use set_menu::SetMenu;
mod set_menu_context_help_id;
pub use set_menu_context_help_id::SetMenuContextHelpId;
mod set_menu_default_item;
pub use set_menu_default_item::SetMenuDefaultItem;
mod set_menu_info;
pub use set_menu_info::SetMenuInfo;
mod set_menu_item_bitmaps;
pub use set_menu_item_bitmaps::SetMenuItemBitmaps;
mod set_message_extra_info;
pub use set_message_extra_info::SetMessageExtraInfo;
mod set_message_queue;
pub use set_message_queue::SetMessageQueue;
mod set_mirror_rendering;
pub use set_mirror_rendering::SetMirrorRendering;
mod set_parent;
pub use set_parent::SetParent;
mod set_physical_cursor_pos;
pub use set_physical_cursor_pos::SetPhysicalCursorPos;
mod set_pointer_device_input_space;
pub use set_pointer_device_input_space::SetPointerDeviceInputSpace;
mod set_process_dpi_aware;
pub use set_process_dpi_aware::SetProcessDPIAware;
mod set_process_default_layout;
pub use set_process_default_layout::SetProcessDefaultLayout;
mod set_process_dpi_awareness_context;
pub use set_process_dpi_awareness_context::SetProcessDpiAwarenessContext;
mod set_process_dpi_awareness_internal;
pub use set_process_dpi_awareness_internal::SetProcessDpiAwarenessInternal;
mod set_process_launch_foreground_policy;
pub use set_process_launch_foreground_policy::SetProcessLaunchForegroundPolicy;
mod set_process_restriction_exemption;
pub use set_process_restriction_exemption::SetProcessRestrictionExemption;
mod set_process_window_station;
pub use set_process_window_station::SetProcessWindowStation;
mod set_progman_window;
pub use set_progman_window::SetProgmanWindow;
mod set_rect;
pub use set_rect::SetRect;
mod set_rect_empty;
pub use set_rect_empty::SetRectEmpty;
mod set_scroll_info;
pub use set_scroll_info::SetScrollInfo;
mod set_scroll_pos;
pub use set_scroll_pos::SetScrollPos;
mod set_scroll_range;
pub use set_scroll_range::SetScrollRange;
mod set_shell_change_notify_window;
pub use set_shell_change_notify_window::SetShellChangeNotifyWindow;
mod set_shell_window;
pub use set_shell_window::SetShellWindow;
mod set_shell_window_ex;
pub use set_shell_window_ex::SetShellWindowEx;
mod set_sys_colors;
pub use set_sys_colors::SetSysColors;
mod set_sys_colors_temp;
pub use set_sys_colors_temp::SetSysColorsTemp;
mod set_system_cursor;
pub use set_system_cursor::SetSystemCursor;
mod set_system_menu;
pub use set_system_menu::SetSystemMenu;
mod set_taskman_window;
pub use set_taskman_window::SetTaskmanWindow;
mod set_thread_cursor_creation_scaling;
pub use set_thread_cursor_creation_scaling::SetThreadCursorCreationScaling;
mod set_thread_desktop;
pub use set_thread_desktop::SetThreadDesktop;
mod set_thread_dpi_awareness_context;
pub use set_thread_dpi_awareness_context::SetThreadDpiAwarenessContext;
mod set_thread_dpi_hosting_behavior;
pub use set_thread_dpi_hosting_behavior::SetThreadDpiHostingBehavior;
mod set_thread_input_blocked;
pub use set_thread_input_blocked::SetThreadInputBlocked;
mod set_timer;
pub use set_timer::SetTimer;
mod set_user_object_capability;
pub use set_user_object_capability::SetUserObjectCapability;
mod set_user_object_security;
pub use set_user_object_security::SetUserObjectSecurity;
mod set_win_event_hook;
pub use set_win_event_hook::SetWinEventHook;
mod set_window_band;
pub use set_window_band::SetWindowBand;
mod set_window_composition_attribute;
pub use set_window_composition_attribute::SetWindowCompositionAttribute;
mod set_window_composition_transition;
pub use set_window_composition_transition::SetWindowCompositionTransition;
mod set_window_context_help_id;
pub use set_window_context_help_id::SetWindowContextHelpId;
mod set_window_display_affinity;
pub use set_window_display_affinity::SetWindowDisplayAffinity;
mod set_window_feedback_setting;
pub use set_window_feedback_setting::SetWindowFeedbackSetting;
mod set_window_message_capability;
pub use set_window_message_capability::SetWindowMessageCapability;
mod set_window_placement;
pub use set_window_placement::SetWindowPlacement;
mod set_window_rgn;
pub use set_window_rgn::SetWindowRgn;
mod set_window_rgn_ex;
pub use set_window_rgn_ex::SetWindowRgnEx;
mod set_window_station_user;
pub use set_window_station_user::SetWindowStationUser;
mod set_window_word;
pub use set_window_word::SetWindowWord;
mod set_windows_hook_ex_aw;
pub use set_windows_hook_ex_aw::SetWindowsHookExAW;
mod shell_foreground_boost_process;
pub use shell_foreground_boost_process::ShellForegroundBoostProcess;
mod shell_handwriting_delegate_input;
pub use shell_handwriting_delegate_input::ShellHandwritingDelegateInput;
mod shell_handwriting_handle_delegated_input;
pub use shell_handwriting_handle_delegated_input::ShellHandwritingHandleDelegatedInput;
mod shell_handwriting_undelegate_input;
pub use shell_handwriting_undelegate_input::ShellHandwritingUndelegateInput;
mod shell_migrate_window;
pub use shell_migrate_window::ShellMigrateWindow;
mod shell_register_hot_key;
pub use shell_register_hot_key::ShellRegisterHotKey;
mod shell_set_window_pos;
pub use shell_set_window_pos::ShellSetWindowPos;
mod show_caret;
pub use show_caret::ShowCaret;
mod show_owned_popups;
pub use show_owned_popups::ShowOwnedPopups;
mod show_scroll_bar;
pub use show_scroll_bar::ShowScrollBar;
mod show_start_glass;
pub use show_start_glass::ShowStartGlass;
mod show_system_cursor;
pub use show_system_cursor::ShowSystemCursor;
mod show_window_async;
pub use show_window_async::ShowWindowAsync;
mod shutdown_block_reason_create;
pub use shutdown_block_reason_create::ShutdownBlockReasonCreate;
mod shutdown_block_reason_destroy;
pub use shutdown_block_reason_destroy::ShutdownBlockReasonDestroy;
mod shutdown_block_reason_query;
pub use shutdown_block_reason_query::ShutdownBlockReasonQuery;
mod signal_redirection_start_complete;
pub use signal_redirection_start_complete::SignalRedirectionStartComplete;
mod skip_pointer_frame_messages;
pub use skip_pointer_frame_messages::SkipPointerFrameMessages;
mod soft_modal_message_box;
pub use soft_modal_message_box::SoftModalMessageBox;
mod sound_sentry;
pub use sound_sentry::SoundSentry;
mod subtract_rect;
pub use subtract_rect::SubtractRect;
mod suppress_window_display_change;
pub use suppress_window_display_change::SuppressWindowDisplayChange;
mod swap_mouse_button;
pub use swap_mouse_button::SwapMouseButton;
mod switch_desktop;
pub use switch_desktop::SwitchDesktop;
mod switch_desktop_with_fade;
pub use switch_desktop_with_fade::SwitchDesktopWithFade;
mod switch_to_this_window;
pub use switch_to_this_window::SwitchToThisWindow;
mod system_parameters_info_for_dpi;
pub use system_parameters_info::SystemParametersInfoW;
pub use system_parameters_info_for_dpi::SystemParametersInfoForDpi;
mod tile_child_windows;
pub use tile_child_windows::TileChildWindows;
mod tile_windows;
pub use tile_windows::TileWindows;
mod to_ascii;
pub use to_ascii::ToAscii;
mod to_ascii_ex;
pub use to_ascii_ex::ToAsciiEx;
mod to_unicode;
pub use to_unicode::ToUnicode;
mod to_unicode_ex;
pub use to_unicode_ex::ToUnicodeEx;
mod track_mouse_event;
pub use track_mouse_event::TrackMouseEvent;
mod track_popup_menu;
pub use track_popup_menu::TrackPopupMenu;
mod track_popup_menu_ex;
pub use track_popup_menu_ex::TrackPopupMenuEx;
mod translate_accelerator;
pub use translate_accelerator::{
    TranslateAccelerator, TranslateAcceleratorA, TranslateAcceleratorW,
};

mod translate_mdi_sys_accel;
pub use translate_mdi_sys_accel::TranslateMDISysAccel;
mod translate_message_ex;
pub use translate_message_ex::TranslateMessageEx;
mod undelegate_input;
pub use undelegate_input::UndelegateInput;
mod unhook_win_event;
pub use unhook_win_event::UnhookWinEvent;
mod unhook_windows_hook;
pub use unhook_windows_hook::UnhookWindowsHook;
mod unhook_windows_hook_ex;
pub use unhook_windows_hook_ex::UnhookWindowsHookEx;
mod union_rect;
pub use union_rect::UnionRect;
mod unload_keyboard_layout;
pub use unload_keyboard_layout::UnloadKeyboardLayout;
mod unlock_window_station;
pub use unlock_window_station::UnlockWindowStation;
mod unpack_dd_el_param;
pub use unpack_dd_el_param::UnpackDDElParam;
mod unregister_device_notification;
pub use unregister_device_notification::UnregisterDeviceNotification;
mod unregister_hot_key;
pub use unregister_hot_key::UnregisterHotKey;
mod unregister_message_pump_hook;
pub use unregister_message_pump_hook::UnregisterMessagePumpHook;
mod unregister_pointer_input_target;
pub use unregister_pointer_input_target::UnregisterPointerInputTarget;
mod unregister_pointer_input_target_ex;
pub use unregister_pointer_input_target_ex::UnregisterPointerInputTargetEx;
mod unregister_power_setting_notification;
pub use unregister_power_setting_notification::UnregisterPowerSettingNotification;
mod unregister_session_port;
pub use unregister_session_port::UnregisterSessionPort;
mod unregister_suspend_resume_notification;
pub use unregister_suspend_resume_notification::UnregisterSuspendResumeNotification;
mod unregister_touch_window;
pub use unregister_touch_window::UnregisterTouchWindow;
mod unregister_user_api_hook;
pub use unregister_user_api_hook::UnregisterUserApiHook;
mod update_default_desktop_thumbnail;
pub use update_default_desktop_thumbnail::UpdateDefaultDesktopThumbnail;
mod update_layered_window;
pub use update_layered_window::UpdateLayeredWindow;
mod update_layered_window_indirect;
pub use update_layered_window_indirect::UpdateLayeredWindowIndirect;
mod update_per_user_system_parameters;
pub use update_per_user_system_parameters::UpdatePerUserSystemParameters;
mod update_window_input_sink_hints;
pub use update_window_input_sink_hints::UpdateWindowInputSinkHints;
mod user32_initialize_imm_entry_table;
pub use user32_initialize_imm_entry_table::User32InitializeImmEntryTable;
mod user_client_dll_initialize;
pub use user_client_dll_initialize::UserClientDllInitialize;
mod user_handle_grant_access;
pub use user_handle_grant_access::UserHandleGrantAccess;
mod user_lpk_psm_text_out;
pub use user_lpk_psm_text_out::UserLpkPSMTextOut;
mod user_lpk_tabbed_text_out;
pub use user_lpk_tabbed_text_out::UserLpkTabbedTextOut;
mod user_realize_palette;
pub use user_realize_palette::UserRealizePalette;
mod user_register_wow_handlers;
pub use user_register_wow_handlers::UserRegisterWowHandlers;
mod v_rip_output;
pub use v_rip_output::VRipOutput;
mod v_tag_output;
pub use v_tag_output::VTagOutput;
mod validate_rect;
pub use validate_rect::ValidateRect;
mod validate_rgn;
pub use validate_rgn::ValidateRgn;
mod wcs_to_mb_ex;
pub use wcs_to_mb_ex::WCSToMBEx;
mod winnls_enable_ime;
pub use winnls_enable_ime::WINNLSEnableIME;
mod winnls_get_enable_status;
pub use winnls_get_enable_status::WINNLSGetEnableStatus;
mod winnls_get_ime_hotkey;
pub use winnls_get_ime_hotkey::WINNLSGetIMEHotkey;
mod wait_for_input_idle;
pub use wait_for_input_idle::WaitForInputIdle;
mod wait_for_redirection_start_complete;
pub use wait_for_redirection_start_complete::WaitForRedirectionStartComplete;
mod wait_message;
pub use wait_message::WaitMessage;
mod window_from_dc;
pub use window_from_dc::WindowFromDC;
mod window_from_physical_point;
pub use window_from_physical_point::WindowFromPhysicalPoint;
mod window_from_point;
pub use window_from_point::WindowFromPoint;
mod _user_test_token_for_interactive;
pub use _user_test_token_for_interactive::_UserTestTokenForInteractive;
mod g_shared_info;
pub use g_shared_info::gSharedInfo;
mod gapfn_sc_send_message;
pub use gapfn_sc_send_message::gapfnScSendMessage;
mod keybd_event;
pub use keybd_event::keybd_event;
mod mouse_event;
pub use mouse_event::mouse_event;
mod wow64_transition;
pub use wow64_transition::Wow64Transition;
mod adjust_window_rect;
use adjust_window_rect::AdjustWindowRect;
mod adjust_window_rect_ex;
use adjust_window_rect_ex::AdjustWindowRectEx;
mod begin_paint;
use begin_paint::BeginPaint;
mod char_lower_buff;
use char_lower_buff::CharLowerBuffA;
use char_lower_buff::CharLowerBuffW;
mod char_upper_buff;
use char_upper_buff::CharUpperBuffA;
use char_upper_buff::CharUpperBuffW;
mod create_window_ex;
use create_window_ex::CreateWindowExA;
use create_window_ex::CreateWindowExW;
mod def_window_proc;
use def_window_proc::DefWindowProcA;
use def_window_proc::DefWindowProcW;
mod destroy_window;
use destroy_window::DestroyWindow;
mod dispatch_message;
use dispatch_message::DispatchMessageA;
use dispatch_message::DispatchMessageW;
mod draw_text;
use draw_text::DrawTextA;
use draw_text::DrawTextW;
mod enable_window;
use enable_window::EnableWindow;
mod end_paint;
use end_paint::EndPaint;
mod enum_windows;
use enum_windows::EnumWindows;
mod fill_rect;
use fill_rect::FillRect;
mod get_async_key_state;
pub use get_async_key_state::GetAsyncKeyState;
mod get_class_info_ex;
use get_class_info_ex::GetClassInfoExA;
use get_class_info_ex::GetClassInfoExW;
mod get_client_rect;
use get_client_rect::GetClientRect;
mod get_cursor_pos;
use get_cursor_pos::GetCursorPos;
mod get_dc;
use get_dc::GetDC;
mod get_desktop_window;
use get_desktop_window::GetDesktopWindow;
mod get_focus;
use get_focus::GetFocus;
mod get_foreground_window;
use get_foreground_window::GetForegroundWindow;
mod get_key_state;
pub use get_key_state::GetKeyState;
mod get_keyboard_layout;
use get_keyboard_layout::GetKeyboardLayout;
mod get_keyboard_type;
use get_keyboard_type::GetKeyboardType;
mod get_message;
use get_message::GetMessageA;
use get_message::GetMessageW;
mod get_physical_cursor_pos;
use get_physical_cursor_pos::GetPhysicalCursorPos;
mod get_process_window_station;
use get_process_window_station::GetProcessWindowStation;
mod get_system_metrics;
use get_system_metrics::GetSystemMetrics;
use get_user_object_information::GetUserObjectInformationW;
mod get_window_long;
use get_window_long::GetWindowLongA;
mod get_window_long_ptr;
use get_window_long::GetWindowLongW;
use get_window_long_ptr::GetWindowLongPtrA;
use get_window_long_ptr::GetWindowLongPtrW;
mod get_window_rect;
use get_window_rect::GetWindowRect;
mod get_window_text;
use get_window_text::GetWindowTextA;
use get_window_text::GetWindowTextW;
mod invalidate_rect;
use invalidate_rect::InvalidateRect;
mod is_window;
use is_window::IsWindow;
mod is_window_visible;
use is_window_visible::IsWindowVisible;
mod load_cursor;
use load_cursor::LoadCursorA;
mod load_icon;
use load_icon::LoadIconA;
mod load_string;
use load_string::LoadStringA;
use load_string::LoadStringW;
mod map_virtual_key;
use map_virtual_key::MapVirtualKeyA;
use map_virtual_key::MapVirtualKeyW;
mod message_box;
use message_box::MessageBoxA;
pub use message_box::MessageBoxW;
mod peek_message;
use peek_message::PeekMessageA;
use peek_message::PeekMessageW;
mod post_message;
use post_message::PostMessageA;
use post_message::PostMessageW;
mod post_quit_message;
use post_quit_message::PostQuitMessage;
mod register_class;
use register_class::RegisterClassA;
mod register_class_ex;
use register_class::RegisterClassW;
use register_class_ex::RegisterClassExA;
use register_class_ex::RegisterClassExW;
mod release_dc;
use release_dc::ReleaseDC;
mod send_message;
use send_message::SendMessageA;
use send_message::SendMessageW;
mod set_cursor;
use set_cursor::SetCursor;
mod set_cursor_pos;
use set_cursor_pos::SetCursorPos;
mod set_focus;
use set_focus::SetFocus;
mod set_foreground_window;
use set_foreground_window::SetForegroundWindow;
mod set_window_long;
use set_window_long::SetWindowLongA;
mod set_window_long_ptr;
use set_window_long::SetWindowLongW;
use set_window_long_ptr::SetWindowLongPtrA;
use set_window_long_ptr::SetWindowLongPtrW;
mod set_window_pos;
use set_window_pos::SetWindowPos;
mod set_window_text;
use set_window_text::SetWindowTextA;
use set_window_text::SetWindowTextW;
mod show_cursor;
use show_cursor::ShowCursor;
mod show_window;
use show_window::ShowWindow;
mod system_parameters_info;
use system_parameters_info::SystemParametersInfoA;
mod translate_message;
use translate_message::TranslateMessage;
mod unregister_class;
use unregister_class::UnregisterClassA;
use unregister_class::UnregisterClassW;
mod update_window;
use update_window::UpdateWindow;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    if strings::dispatch(api, "user32", emu) {
        return String::new();
    }
    if clipboard::dispatch(api, addr, emu)
        || dde::dispatch(api, addr, emu)
        || dialogs::dispatch(api, addr, emu)
        || graphics::dispatch(api, addr, emu)
        || input::dispatch(api, addr, emu)
        || menus::dispatch(api, addr, emu)
        || messages::dispatch(api, addr, emu)
        || strings_format::dispatch(api, addr, emu)
        || system::dispatch(api, addr, emu)
        || windowing::dispatch(api, addr, emu)
    {
        return String::new();
    }

    fallback::unimplemented(addr, api, emu)
}

/*
int GetSystemMetrics(
  [in] int nIndex
);
*/
/*
BOOL SystemParametersInfoA(
  [in]      UINT  uiAction,
  [in]      UINT  uiParam,
  [in, out] PVOID pvParam,
  [in]      UINT  fWinIni
);
*/
/*
HICON LoadIconA(
[in, optional] HINSTANCE hInstance,
[in]           LPCSTR    lpIconName
);
*/
/*
HCURSOR LoadCursorA(
  [in, optional] HINSTANCE hInstance,
  [in]           LPCSTR    lpCursorName
);
*/
/*
ATOM RegisterClassA(
  [in] const WNDCLASSA *lpWndClass
);
*/
/*
ATOM RegisterClassExA(
  [in] const WNDCLASSEXA *unnamedParam1
);
*/
/*
ATOM RegisterClassExW(
  [in] const WNDCLASSEXW *unnamedParam1
);
*/
/*
BOOL UnregisterClassW(
  [in]           LPCWSTR   lpClassName,
  [in, optional] HINSTANCE hInstance
);
*/
/// Shared body of UnregisterClass{A,W}: TRUE in rax when the class was removed.
fn unregister_class(emu: &mut emu::Emu, api: &str, wide: bool) {
    let class_ptr = emu.regs().rcx;
    let name = class_arg(emu, class_ptr, wide).unwrap_or_default();
    log_red!(emu, "user32!{} class:'{}'", api, name);
    let error = match window_table::unregister_class(&name) {
        Ok(()) => {
            emu.regs_mut().rax = 1;
            return;
        }
        Err(window_table::UnregisterError::NotRegistered) => ERROR_CLASS_DOES_NOT_EXIST,
        Err(window_table::UnregisterError::HasWindows) => ERROR_CLASS_HAS_WINDOWS,
    };
    emu.set_last_error(error);
    emu.regs_mut().rax = 0;
}

/// WNDCLASS and WNDCLASSEX both keep lpszClassName at +0x40 on x64.
const WNDCLASS_CLASS_NAME_OFF: u64 = 0x40;

/// Shared body of RegisterClass[Ex]{A,W}: returns the class ATOM in rax.
fn register_class(emu: &mut emu::Emu, lpwndclass: u64, wide: bool) {
    let name_ptr = match emu.maps.read_qword(lpwndclass + WNDCLASS_CLASS_NAME_OFF) {
        Some(ptr) if lpwndclass != 0 => ptr,
        _ => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    match class_arg(emu, name_ptr, wide) {
        Some(name) => emu.regs_mut().rax = window_table::register_class(&name) as u64,
        None => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            emu.regs_mut().rax = 0;
        }
    }
}

/// lpClassName as CreateWindowEx/FindWindow take it: NULL, an ATOM
/// (MAKEINTATOM, below 0x10000) or a string.
fn class_arg(emu: &mut emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        return None;
    }
    if ptr < 0x10000 {
        return Some(window_table::class_name(ptr as u16).unwrap_or_else(|| format!("#{}", ptr)));
    }
    Some(read_text(emu, ptr, wide))
}

/// Optional window title argument (NULL = no filter).
fn title_arg(emu: &mut emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        None
    } else {
        Some(read_text(emu, ptr, wide))
    }
}

fn read_text(emu: &mut emu::Emu, ptr: u64, wide: bool) -> String {
    strings::read_text(emu, ptr, wide)
}

/*
BOOL EnumWindows(
  [in] WNDENUMPROC lpEnumFunc,
  [in] LPARAM      lParam
);
*/
/*
HWINSTA GetProcessWindowStation();
*/
const UOI_FLAGS: u64 = 1;
const UOI_NAME: u64 = 2;
const UOI_TYPE: u64 = 3;
const WSF_VISIBLE: u32 = 1;

/*
BOOL GetUserObjectInformationW(
  [in]            HANDLE  hObj,
  [in]            int     nIndex,
  [out, optional] PVOID   pvInfo,
  [in]            DWORD   nLength,
  [out, optional] LPDWORD lpnLengthNeeded
);
*/
/// UTF-16LE bytes of `s` including the terminating NUL.
fn wide_z(s: &str) -> Vec<u8> {
    s.encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(|c| c.to_le_bytes())
        .collect()
}

/*
HDC GetDC(
  [in] HWND hWnd
);
*/
/*
int ReleaseDC(
  [in] HWND hWnd,
  [in] HDC  hDC
);
*/
/*
HWND FindWindowExA(
  [in, optional] HWND   hWndParent,
  [in, optional] HWND   hWndChildAfter,
  [in, optional] LPCSTR lpszClass,
  [in, optional] LPCSTR lpszWindow
);
*/
/// Shared body of FindWindow[Ex]{A,W}: the HWND in rax, 0 when not found.
fn find_window(
    emu: &mut emu::Emu,
    api: &str,
    parent: u64,
    after: u64,
    class_ptr: u64,
    window_ptr: u64,
    wide: bool,
) {
    let class_name = class_arg(emu, class_ptr, wide);
    let window_name = title_arg(emu, window_ptr, wide);
    let hwnd =
        window_table::find_window(parent, after, class_name.as_deref(), window_name.as_deref());

    log_red!(
        emu,
        "user32!{} parent=0x{:x} after=0x{:x} class={:?} window={:?} = 0x{:x}",
        api,
        parent,
        after,
        class_name,
        window_name,
        hwnd
    );

    emu.regs_mut().rax = hwnd;
}

/// Shared body of CreateWindowEx{A,W}: records the window, HWND in rax.
fn create_window(emu: &mut emu::Emu, api: &str, wide: bool) {
    let (class_ptr, title_ptr) = (emu.regs().rdx, emu.regs().r8);
    let parent = WinApiAbi::arg(emu, 8);
    let class = class_arg(emu, class_ptr, wide).unwrap_or_default();
    let title = title_arg(emu, title_ptr, wide).unwrap_or_default();
    let hwnd = window_table::create_window(parent, &class, &title);
    log_red!(
        emu,
        "user32!{} class:'{}' title:'{}' parent:0x{:x} = 0x{:x}",
        api,
        class,
        title,
        parent,
        hwnd
    );
    emu.regs_mut().rax = hwnd;
}

fn get_window_long_value(index: i32) -> u64 {
    match index {
        -16 => 0x00CF0000, // GWL_STYLE -> WS_OVERLAPPEDWINDOW
        -20 => 0x00000100, // GWL_EXSTYLE -> WS_EX_WINDOWEDGE
        -6 => 0,           // GWL_HINSTANCE
        -12 => 0,          // GWL_ID
        -21 => 0,          // GWL_USERDATA
        -4 => 0,           // GWL_WNDPROC
        _ => 0,
    }
}

/// POINT at rcx = the centre of the 1920x1080 desktop; TRUE in rax.
fn write_cursor_pos(emu: &mut emu::Emu) {
    let pt = emu.regs().rcx;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    emu.regs_mut().rax = 1;
}
