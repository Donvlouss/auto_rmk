use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CallNextHookEx, PeekMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, HHOOK,
        KBDLLHOOKSTRUCT, MSG, MSLLHOOKSTRUCT, PM_NOREMOVE, WH_KEYBOARD_LL, WH_MOUSE_LL, WM_KEYDOWN,
        WM_KEYUP,
    },
};

use crate::core::{Cursor, CursorAction, Keyboard, MKAction};

static mut QUEUE: Vec<MKAction> = vec![];

#[derive(Debug)]
pub struct Record {
    thread: Option<JoinHandle<()>>,
    stop: Option<Arc<AtomicBool>>,
    hook: Vec<HHOOK>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            thread: None,
            stop: None,
            hook: Vec::with_capacity(2),
        }
    }

    pub fn register_hook(&mut self) {
        self.hook = unsafe {
            vec![
                SetWindowsHookExW(WH_MOUSE_LL, Some(hook_proc), HINSTANCE::default(), 0).unwrap(),
                SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), HINSTANCE::default(), 0)
                    .unwrap(),
            ]
        }
    }

    pub fn unregister_hook(&mut self) {
        for hook in self.hook.iter() {
            unsafe { UnhookWindowsHookEx(*hook).unwrap() };
        }
        self.hook.clear();
    }

    pub fn start(&mut self) {
        self.register_hook();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();
        let handle = std::thread::spawn(move || unsafe {
            QUEUE.clear();
            println!("Start");
            let mut msg = MSG::default();
            while !stop_clone.load(std::sync::atomic::Ordering::Relaxed) {
                let _ = PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE);
            }
        });
        self.stop = Some(stop);
        self.thread = Some(handle);
    }

    pub fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            if let Some(handle) = self.thread.take() {
                handle.join().unwrap();
                self.unregister_hook();
                println!("Stop");
            }
        }
    }

    pub fn get_actions(&self) -> Vec<MKAction> {
        unsafe { QUEUE.clone() }
    }
}

unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let w = w_param.0 as u32;
        if w >= CursorAction::Move as u32 && w <= CursorAction::Wheel as u32 {
            let ms_struct = *(l_param.0 as *const MSLLHOOKSTRUCT);
            QUEUE.push(MKAction::Mouse(Cursor {
                x: ms_struct.pt.x,
                y: ms_struct.pt.y,
                action: CursorAction::from_wm(w),
                timestamp: chrono::Utc::now().timestamp_subsec_millis(),
            }));
        } else if w == WM_KEYDOWN || w == WM_KEYUP {
            let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            QUEUE.push(MKAction::Keyboard(Keyboard {
                key: kb_struct.vkCode as u16,
                down: w == WM_KEYDOWN,
                timestamp: chrono::Utc::now().timestamp_subsec_millis(),
            }));
        }
    }
    CallNextHookEx(None, n_code, w_param, l_param)
}
