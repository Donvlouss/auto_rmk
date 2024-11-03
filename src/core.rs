use windows::Win32::UI::WindowsAndMessaging::{
    WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL,
    WM_RBUTTONDOWN, WM_RBUTTONUP,
};

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub action: Option<CursorAction>,
    pub timestamp: u32,
}

impl Cursor {
    pub fn update(mut self, action: CursorAction) -> Self {
        self.action = Some(action);
        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum CursorAction {
    Move = WM_MOUSEMOVE,
    LDown = WM_LBUTTONDOWN,
    LUp = WM_LBUTTONUP,
    RDown = WM_RBUTTONDOWN,
    RUp = WM_RBUTTONUP,
    MDown = WM_MBUTTONDOWN,
    MUp = WM_MBUTTONUP,
    Wheel = WM_MOUSEWHEEL,
}

impl CursorAction {
    pub fn from_wm(c: u32) -> Option<Self> {
        match c {
            x if x == CursorAction::Move as u32 => Some(CursorAction::Move),
            x if x == CursorAction::LDown as u32 => Some(CursorAction::LDown),
            x if x == CursorAction::LUp as u32 => Some(CursorAction::LUp),
            x if x == CursorAction::RDown as u32 => Some(CursorAction::RDown),
            x if x == CursorAction::RUp as u32 => Some(CursorAction::RUp),
            x if x == CursorAction::MDown as u32 => Some(CursorAction::MDown),
            x if x == CursorAction::MUp as u32 => Some(CursorAction::MUp),
            x if x == CursorAction::Wheel as u32 => Some(CursorAction::Wheel),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Keyboard {
    pub key: u16,
    pub down: bool,
    pub timestamp: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum MKAction {
    Mouse(Cursor),
    Keyboard(Keyboard),
}
