// Windows System Tray integration for Rapid Download Manager
#![cfg(windows)]

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

#[repr(C)]
pub struct NOTIFYICONDATAW {
    pub cb_size: u32,
    pub h_wnd: isize,
    pub u_id: u32,
    pub u_flags: u32,
    pub u_callback_message: u32,
    pub h_icon: isize,
    pub sz_tip: [u16; 128],
    pub dw_state: u32,
    pub dw_state_mask: u32,
    pub sz_info: [u16; 256],
    pub u_timeout_or_version: u32,
    pub sz_info_title: [u16; 64],
    pub dw_info_flags: u32,
    pub guid_item: [u8; 16],
    pub h_balloon_icon: isize,
}

#[repr(C)]
pub struct WNDCLASSEXW {
    pub cb_size: u32,
    pub style: u32,
    pub lpfn_wnd_proc: unsafe extern "system" fn(isize, u32, usize, isize) -> isize,
    pub cb_cls_extra: i32,
    pub cb_wnd_extra: i32,
    pub h_instance: isize,
    pub h_icon: isize,
    pub h_cursor: isize,
    pub hbr_background: isize,
    pub lpsz_menu_name: *const u16,
    pub lpsz_class_name: *const u16,
    pub h_icon_sm: isize,
}

#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct MSG {
    pub h_wnd: isize,
    pub message: u32,
    pub w_param: usize,
    pub l_param: isize,
    pub time: u32,
    pub pt: POINT,
}

pub const NIM_ADD: u32 = 0x00000000;
pub const NIM_MODIFY: u32 = 0x00000001;
pub const NIM_DELETE: u32 = 0x00000002;

pub const NIF_MESSAGE: u32 = 0x00000001;
pub const NIF_ICON: u32 = 0x00000002;
pub const NIF_TIP: u32 = 0x00000004;

pub const WM_USER: u32 = 0x0400;
pub const WM_TRAYICON: u32 = WM_USER + 101;
pub const WM_UPDATE_TIP: u32 = WM_USER + 102;
pub const WM_DESTROY: u32 = 0x0002;

pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_LBUTTONDBLCLK: u32 = 0x0203;
pub const WM_RBUTTONUP: u32 = 0x0205;

pub const TPM_BOTTOMALIGN: u32 = 0x0020;
pub const TPM_LEFTALIGN: u32 = 0x0000;
pub const TPM_RETURNCMD: u32 = 0x0100;
pub const MF_STRING: u32 = 0x0000;
pub const MF_SEPARATOR: u32 = 0x0800;

pub const IDI_APPLICATION: usize = 32512;
pub const SW_RESTORE: i32 = 9;
pub const SW_SHOW: i32 = 5;

#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconW(dwMessage: u32, lpData: *const NOTIFYICONDATAW) -> i32;
}

#[link(name = "user32")]
extern "system" {
    fn DefWindowProcW(hWnd: isize, Msg: u32, wParam: usize, lParam: isize) -> isize;
    fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
    fn CreateWindowExW(
        dwExStyle: u32,
        lpClassName: *const u16,
        lpWindowName: *const u16,
        dwStyle: u32,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: isize,
        hMenu: isize,
        hInstance: isize,
        lpParam: *mut std::ffi::c_void,
    ) -> isize;
    fn DestroyWindow(hWnd: isize) -> i32;
    fn PostQuitMessage(nExitCode: i32);
    fn PostMessageW(hWnd: isize, Msg: u32, wParam: usize, lParam: isize) -> i32;
    fn GetMessageW(lpMsg: *mut MSG, hWnd: isize, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
    fn TranslateMessage(lpMsg: *const MSG) -> i32;
    fn DispatchMessageW(lpMsg: *const MSG) -> isize;
    fn LoadIconW(hInstance: isize, lpIconName: *const u16) -> isize;
    fn CreateIconFromResourceEx(
        pbIconBits: *const u8,
        cbIconBits: u32,
        fIcon: i32,
        dwVersion: u32,
        cxDesired: i32,
        cyDesired: i32,
        uFlags: u32,
    ) -> isize;
    fn CreatePopupMenu() -> isize;
    fn AppendMenuW(hMenu: isize, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> i32;
    fn TrackPopupMenu(
        hMenu: isize,
        uFlags: u32,
        x: i32,
        y: i32,
        nReserved: i32,
        hWnd: isize,
        prcRect: *const std::ffi::c_void,
    ) -> i32;
    fn DestroyMenu(hMenu: isize) -> i32;
    fn GetCursorPos(lpPoint: *mut POINT) -> i32;
    fn SetForegroundWindow(hWnd: isize) -> i32;
    fn ShowWindow(hWnd: isize, nCmdShow: i32) -> i32;
    fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> isize;
    fn GetModuleHandleW(lpModuleName: *const u16) -> isize;
}

static RESTORE_FLAG: AtomicBool = AtomicBool::new(false);
static PAUSE_ALL_FLAG: AtomicBool = AtomicBool::new(false);
static RESUME_ALL_FLAG: AtomicBool = AtomicBool::new(false);
static TRAY_HWND: AtomicIsize = AtomicIsize::new(0);
static CURRENT_TIP: Mutex<Option<String>> = Mutex::new(None);

unsafe extern "system" fn tray_window_proc(
    h_wnd: isize,
    msg: u32,
    w_param: usize,
    l_param: isize,
) -> isize {
    match msg {
        WM_TRAYICON => {
            let event = (l_param & 0xFFFF) as u32;
            match event {
                WM_LBUTTONUP | WM_LBUTTONDBLCLK => {
                    RESTORE_FLAG.store(true, Ordering::Relaxed);
                    // Bring main window to front
                    let window_title: Vec<u16> = "Rapid Download Manager\0".encode_utf16().collect();
                    let target_hwnd = FindWindowW(std::ptr::null(), window_title.as_ptr());
                    if target_hwnd != 0 {
                        ShowWindow(target_hwnd, SW_RESTORE);
                        ShowWindow(target_hwnd, SW_SHOW);
                        SetForegroundWindow(target_hwnd);
                    }
                }
                WM_RBUTTONUP => {
                    let mut pt = POINT { x: 0, y: 0 };
                    GetCursorPos(&mut pt);
                    let h_menu = CreatePopupMenu();
                    let open_text: Vec<u16> = "[RDM] Open Rapid Download Manager\0".encode_utf16().collect();
                    let pause_text: Vec<u16> = "[||] Pause All\0".encode_utf16().collect();
                    let resume_text: Vec<u16> = "[>] Resume All\0".encode_utf16().collect();
                    let exit_text: Vec<u16> = "[X] Exit Application\0".encode_utf16().collect();

                    AppendMenuW(h_menu, MF_STRING, 1, open_text.as_ptr());
                    AppendMenuW(h_menu, MF_SEPARATOR, 0, std::ptr::null());
                    AppendMenuW(h_menu, MF_STRING, 3, pause_text.as_ptr());
                    AppendMenuW(h_menu, MF_STRING, 4, resume_text.as_ptr());
                    AppendMenuW(h_menu, MF_SEPARATOR, 0, std::ptr::null());
                    AppendMenuW(h_menu, MF_STRING, 2, exit_text.as_ptr());

                    SetForegroundWindow(h_wnd);
                    let cmd = TrackPopupMenu(
                        h_menu,
                        TPM_BOTTOMALIGN | TPM_LEFTALIGN | TPM_RETURNCMD,
                        pt.x,
                        pt.y,
                        0,
                        h_wnd,
                        std::ptr::null(),
                    );
                    DestroyMenu(h_menu);

                    if cmd == 1 {
                        RESTORE_FLAG.store(true, Ordering::Relaxed);
                        let window_title: Vec<u16> = "Rapid Download Manager\0".encode_utf16().collect();
                        let target_hwnd = FindWindowW(std::ptr::null(), window_title.as_ptr());
                        if target_hwnd != 0 {
                            ShowWindow(target_hwnd, SW_RESTORE);
                            ShowWindow(target_hwnd, SW_SHOW);
                            SetForegroundWindow(target_hwnd);
                        }
                    } else if cmd == 3 {
                        PAUSE_ALL_FLAG.store(true, Ordering::Relaxed);
                    } else if cmd == 4 {
                        RESUME_ALL_FLAG.store(true, Ordering::Relaxed);
                    } else if cmd == 2 {
                        std::process::exit(0);
                    }
                }
                _ => {}
            }
            0
        }
        WM_UPDATE_TIP => {
            if let Ok(guard) = CURRENT_TIP.lock() {
                if let Some(ref text) = *guard {
                    let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
                    nid.cb_size = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                    nid.h_wnd = h_wnd;
                    nid.u_id = 1;
                    nid.u_flags = NIF_TIP;
                    let tip_utf16: Vec<u16> = format!("{}\0", text).encode_utf16().collect();
                    for (idx, &c) in tip_utf16.iter().enumerate().take(127) {
                        nid.sz_tip[idx] = c;
                    }
                    Shell_NotifyIconW(NIM_MODIFY, &nid);
                }
            }
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(h_wnd, msg, w_param, l_param),
    }
}

pub struct TrayHandle {
    pub restore_signal: Arc<AtomicBool>,
    pub pause_all_signal: Arc<AtomicBool>,
    pub resume_all_signal: Arc<AtomicBool>,
}

impl TrayHandle {
    pub fn new() -> Self {
        let restore_signal = Arc::new(AtomicBool::new(false));
        let pause_all_signal = Arc::new(AtomicBool::new(false));
        let resume_all_signal = Arc::new(AtomicBool::new(false));

        let restore_clone = Arc::clone(&restore_signal);
        let pause_clone = Arc::clone(&pause_all_signal);
        let resume_clone = Arc::clone(&resume_all_signal);

        thread::spawn(move || unsafe {
            let class_name: Vec<u16> = "RapidDownloadManagerTrayClass\0".encode_utf16().collect();
            let h_inst = GetModuleHandleW(std::ptr::null());
            let png_bytes = include_bytes!("../assets/icon.png");
            let custom_icon = CreateIconFromResourceEx(
                png_bytes.as_ptr(),
                png_bytes.len() as u32,
                1,
                0x00030000,
                32,
                32,
                0,
            );
            let icon = if custom_icon != 0 {
                custom_icon
            } else {
                LoadIconW(0, IDI_APPLICATION as *const u16)
            };

            let wnd_class = WNDCLASSEXW {
                cb_size: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: 0,
                lpfn_wnd_proc: tray_window_proc,
                cb_cls_extra: 0,
                cb_wnd_extra: 0,
                h_instance: h_inst,
                h_icon: icon,
                h_cursor: 0,
                hbr_background: 0,
                lpsz_menu_name: std::ptr::null(),
                lpsz_class_name: class_name.as_ptr(),
                h_icon_sm: icon,
            };

            RegisterClassExW(&wnd_class);

            let h_wnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                class_name.as_ptr(),
                0,
                0,
                0,
                0,
                0,
                -3, // HWND_MESSAGE (message-only window, prevents desktop window interference)
                0,
                h_inst,
                std::ptr::null_mut(),
            );

            if h_wnd == 0 {
                return;
            }

            TRAY_HWND.store(h_wnd, Ordering::SeqCst);

            let mut nid = std::mem::zeroed::<NOTIFYICONDATAW>();
            nid.cb_size = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.h_wnd = h_wnd;
            nid.u_id = 1;
            nid.u_flags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
            nid.u_callback_message = WM_TRAYICON;
            nid.h_icon = icon;

            let tip_utf16: Vec<u16> = "Rapid Download Manager - Ready & Intercepting\0".encode_utf16().collect();
            for (idx, &c) in tip_utf16.iter().enumerate().take(127) {
                nid.sz_tip[idx] = c;
            }

            Shell_NotifyIconW(NIM_ADD, &nid);

            let mut msg = std::mem::zeroed::<MSG>();
            while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);

                if RESTORE_FLAG.swap(false, Ordering::Relaxed) {
                    restore_clone.store(true, Ordering::Relaxed);
                }
                if PAUSE_ALL_FLAG.swap(false, Ordering::Relaxed) {
                    pause_clone.store(true, Ordering::Relaxed);
                }
                if RESUME_ALL_FLAG.swap(false, Ordering::Relaxed) {
                    resume_clone.store(true, Ordering::Relaxed);
                }
            }

            Shell_NotifyIconW(NIM_DELETE, &nid);
            DestroyWindow(h_wnd);
        });

        Self {
            restore_signal,
            pause_all_signal,
            resume_all_signal,
        }
    }

    pub fn should_restore(&self) -> bool {
        self.restore_signal.swap(false, Ordering::Relaxed)
    }

    pub fn should_pause_all(&self) -> bool {
        self.pause_all_signal.swap(false, Ordering::Relaxed)
    }

    pub fn should_resume_all(&self) -> bool {
        self.resume_all_signal.swap(false, Ordering::Relaxed)
    }

    pub fn update_tooltip(&self, text: &str) {
        if let Ok(mut guard) = CURRENT_TIP.lock() {
            *guard = Some(text.to_string());
        }
        let hwnd = TRAY_HWND.load(Ordering::SeqCst);
        if hwnd != 0 {
            unsafe {
                PostMessageW(hwnd, WM_UPDATE_TIP, 0, 0);
            }
        }
    }
}
