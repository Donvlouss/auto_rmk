use std::{
    sync::{
        atomic::AtomicBool,
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::VK_F2,
        WindowsAndMessaging::{
            CallNextHookEx, PeekMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, HHOOK,
            KBDLLHOOKSTRUCT, MSG, PM_NOREMOVE, WH_KEYBOARD_LL, WM_KEYUP,
        },
    },
};

static mut TX: Option<Sender<()>> = None;

#[derive(Debug, Default)]
pub struct GlobalKey {
    pub stop: Option<Arc<AtomicBool>>,
    thread: Option<JoinHandle<()>>,
    hook: Option<HHOOK>,
}

impl GlobalKey {
    pub fn start(&mut self) -> Receiver<()> {
        self.hook = unsafe {
            Some(
                SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_proc), HINSTANCE::default(), 0)
                    .unwrap(),
            )
        };
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();
        let (tx, rx) = channel::<()>();
        let handle = std::thread::spawn(move || unsafe {
            TX = Some(tx);
            let mut msg = MSG::default();
            while !stop_clone.load(std::sync::atomic::Ordering::Relaxed) {
                let _ = PeekMessageW(&mut msg, None, 0, 0, PM_NOREMOVE);
            }
        });
        self.stop = Some(stop);
        self.thread = Some(handle);
        rx
    }

    pub fn stop(&mut self) {
        if let Some(stop) = self.stop.take() {
            stop.store(true, std::sync::atomic::Ordering::Relaxed);
            if let Some(handle) = self.thread.take() {
                handle.join().unwrap();
                unsafe { UnhookWindowsHookEx(self.hook.unwrap()).unwrap() }
            }
        }
    }
}

#[allow(static_mut_refs)]
unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let w = w_param.0 as u32;
        if w == WM_KEYUP {
            let kb_struct = *(l_param.0 as *const KBDLLHOOKSTRUCT);
            if kb_struct.vkCode as u16 == VK_F2.0 {
                if let Some(s) = &TX {
                    s.send(()).unwrap();
                }
            }
        }
    }
    CallNextHookEx(None, n_code, w_param, l_param)
}
