#![allow(unreachable_patterns)]

use std::ffi::c_char;
use std::ffi::{CStr, CString};
use std::mem::transmute;

use crate::sys;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Keycode {
    Backspace = sys::SDL_KeyCode::SDLK_BACKSPACE as i32,
    Tab = sys::SDL_KeyCode::SDLK_TAB as i32,
    Return = sys::SDL_KeyCode::SDLK_RETURN as i32,
    Escape = sys::SDL_KeyCode::SDLK_ESCAPE as i32,
    Space = sys::SDL_KeyCode::SDLK_SPACE as i32,
    Exclaim = sys::SDL_KeyCode::SDLK_EXCLAIM as i32,
    Quotedbl = sys::SDL_KeyCode::SDLK_QUOTEDBL as i32,
    Hash = sys::SDL_KeyCode::SDLK_HASH as i32,
    Dollar = sys::SDL_KeyCode::SDLK_DOLLAR as i32,
    Percent = sys::SDL_KeyCode::SDLK_PERCENT as i32,
    Ampersand = sys::SDL_KeyCode::SDLK_AMPERSAND as i32,
    Quote = sys::SDL_KeyCode::SDLK_QUOTE as i32,
    LeftParen = sys::SDL_KeyCode::SDLK_LEFTPAREN as i32,
    RightParen = sys::SDL_KeyCode::SDLK_RIGHTPAREN as i32,
    Asterisk = sys::SDL_KeyCode::SDLK_ASTERISK as i32,
    Plus = sys::SDL_KeyCode::SDLK_PLUS as i32,
    Comma = sys::SDL_KeyCode::SDLK_COMMA as i32,
    Minus = sys::SDL_KeyCode::SDLK_MINUS as i32,
    Period = sys::SDL_KeyCode::SDLK_PERIOD as i32,
    Slash = sys::SDL_KeyCode::SDLK_SLASH as i32,
    Num0 = sys::SDL_KeyCode::SDLK_0 as i32,
    Num1 = sys::SDL_KeyCode::SDLK_1 as i32,
    Num2 = sys::SDL_KeyCode::SDLK_2 as i32,
    Num3 = sys::SDL_KeyCode::SDLK_3 as i32,
    Num4 = sys::SDL_KeyCode::SDLK_4 as i32,
    Num5 = sys::SDL_KeyCode::SDLK_5 as i32,
    Num6 = sys::SDL_KeyCode::SDLK_6 as i32,
    Num7 = sys::SDL_KeyCode::SDLK_7 as i32,
    Num8 = sys::SDL_KeyCode::SDLK_8 as i32,
    Num9 = sys::SDL_KeyCode::SDLK_9 as i32,
    Colon = sys::SDL_KeyCode::SDLK_COLON as i32,
    Semicolon = sys::SDL_KeyCode::SDLK_SEMICOLON as i32,
    Less = sys::SDL_KeyCode::SDLK_LESS as i32,
    Equals = sys::SDL_KeyCode::SDLK_EQUALS as i32,
    Greater = sys::SDL_KeyCode::SDLK_GREATER as i32,
    Question = sys::SDL_KeyCode::SDLK_QUESTION as i32,
    At = sys::SDL_KeyCode::SDLK_AT as i32,
    LeftBracket = sys::SDL_KeyCode::SDLK_LEFTBRACKET as i32,
    Backslash = sys::SDL_KeyCode::SDLK_BACKSLASH as i32,
    RightBracket = sys::SDL_KeyCode::SDLK_RIGHTBRACKET as i32,
    Caret = sys::SDL_KeyCode::SDLK_CARET as i32,
    Underscore = sys::SDL_KeyCode::SDLK_UNDERSCORE as i32,
    Backquote = sys::SDL_KeyCode::SDLK_BACKQUOTE as i32,
    A = sys::SDL_KeyCode::SDLK_a as i32,
    B = sys::SDL_KeyCode::SDLK_b as i32,
    C = sys::SDL_KeyCode::SDLK_c as i32,
    D = sys::SDL_KeyCode::SDLK_d as i32,
    E = sys::SDL_KeyCode::SDLK_e as i32,
    F = sys::SDL_KeyCode::SDLK_f as i32,
    G = sys::SDL_KeyCode::SDLK_g as i32,
    H = sys::SDL_KeyCode::SDLK_h as i32,
    I = sys::SDL_KeyCode::SDLK_i as i32,
    J = sys::SDL_KeyCode::SDLK_j as i32,
    K = sys::SDL_KeyCode::SDLK_k as i32,
    L = sys::SDL_KeyCode::SDLK_l as i32,
    M = sys::SDL_KeyCode::SDLK_m as i32,
    N = sys::SDL_KeyCode::SDLK_n as i32,
    O = sys::SDL_KeyCode::SDLK_o as i32,
    P = sys::SDL_KeyCode::SDLK_p as i32,
    Q = sys::SDL_KeyCode::SDLK_q as i32,
    R = sys::SDL_KeyCode::SDLK_r as i32,
    S = sys::SDL_KeyCode::SDLK_s as i32,
    T = sys::SDL_KeyCode::SDLK_t as i32,
    U = sys::SDL_KeyCode::SDLK_u as i32,
    V = sys::SDL_KeyCode::SDLK_v as i32,
    W = sys::SDL_KeyCode::SDLK_w as i32,
    X = sys::SDL_KeyCode::SDLK_x as i32,
    Y = sys::SDL_KeyCode::SDLK_y as i32,
    Z = sys::SDL_KeyCode::SDLK_z as i32,
    Delete = sys::SDL_KeyCode::SDLK_DELETE as i32,
    CapsLock = sys::SDL_KeyCode::SDLK_CAPSLOCK as i32,
    F1 = sys::SDL_KeyCode::SDLK_F1 as i32,
    F2 = sys::SDL_KeyCode::SDLK_F2 as i32,
    F3 = sys::SDL_KeyCode::SDLK_F3 as i32,
    F4 = sys::SDL_KeyCode::SDLK_F4 as i32,
    F5 = sys::SDL_KeyCode::SDLK_F5 as i32,
    F6 = sys::SDL_KeyCode::SDLK_F6 as i32,
    F7 = sys::SDL_KeyCode::SDLK_F7 as i32,
    F8 = sys::SDL_KeyCode::SDLK_F8 as i32,
    F9 = sys::SDL_KeyCode::SDLK_F9 as i32,
    F10 = sys::SDL_KeyCode::SDLK_F10 as i32,
    F11 = sys::SDL_KeyCode::SDLK_F11 as i32,
    F12 = sys::SDL_KeyCode::SDLK_F12 as i32,
    PrintScreen = sys::SDL_KeyCode::SDLK_PRINTSCREEN as i32,
    ScrollLock = sys::SDL_KeyCode::SDLK_SCROLLLOCK as i32,
    Pause = sys::SDL_KeyCode::SDLK_PAUSE as i32,
    Insert = sys::SDL_KeyCode::SDLK_INSERT as i32,
    Home = sys::SDL_KeyCode::SDLK_HOME as i32,
    PageUp = sys::SDL_KeyCode::SDLK_PAGEUP as i32,
    End = sys::SDL_KeyCode::SDLK_END as i32,
    PageDown = sys::SDL_KeyCode::SDLK_PAGEDOWN as i32,
    Right = sys::SDL_KeyCode::SDLK_RIGHT as i32,
    Left = sys::SDL_KeyCode::SDLK_LEFT as i32,
    Down = sys::SDL_KeyCode::SDLK_DOWN as i32,
    Up = sys::SDL_KeyCode::SDLK_UP as i32,
    NumLockClear = sys::SDL_KeyCode::SDLK_NUMLOCKCLEAR as i32,
    KpDivide = sys::SDL_KeyCode::SDLK_KP_DIVIDE as i32,
    KpMultiply = sys::SDL_KeyCode::SDLK_KP_MULTIPLY as i32,
    KpMinus = sys::SDL_KeyCode::SDLK_KP_MINUS as i32,
    KpPlus = sys::SDL_KeyCode::SDLK_KP_PLUS as i32,
    KpEnter = sys::SDL_KeyCode::SDLK_KP_ENTER as i32,
    Kp1 = sys::SDL_KeyCode::SDLK_KP_1 as i32,
    Kp2 = sys::SDL_KeyCode::SDLK_KP_2 as i32,
    Kp3 = sys::SDL_KeyCode::SDLK_KP_3 as i32,
    Kp4 = sys::SDL_KeyCode::SDLK_KP_4 as i32,
    Kp5 = sys::SDL_KeyCode::SDLK_KP_5 as i32,
    Kp6 = sys::SDL_KeyCode::SDLK_KP_6 as i32,
    Kp7 = sys::SDL_KeyCode::SDLK_KP_7 as i32,
    Kp8 = sys::SDL_KeyCode::SDLK_KP_8 as i32,
    Kp9 = sys::SDL_KeyCode::SDLK_KP_9 as i32,
    Kp0 = sys::SDL_KeyCode::SDLK_KP_0 as i32,
    KpPeriod = sys::SDL_KeyCode::SDLK_KP_PERIOD as i32,
    Application = sys::SDL_KeyCode::SDLK_APPLICATION as i32,
    Power = sys::SDL_KeyCode::SDLK_POWER as i32,
    KpEquals = sys::SDL_KeyCode::SDLK_KP_EQUALS as i32,
    F13 = sys::SDL_KeyCode::SDLK_F13 as i32,
    F14 = sys::SDL_KeyCode::SDLK_F14 as i32,
    F15 = sys::SDL_KeyCode::SDLK_F15 as i32,
    F16 = sys::SDL_KeyCode::SDLK_F16 as i32,
    F17 = sys::SDL_KeyCode::SDLK_F17 as i32,
    F18 = sys::SDL_KeyCode::SDLK_F18 as i32,
    F19 = sys::SDL_KeyCode::SDLK_F19 as i32,
    F20 = sys::SDL_KeyCode::SDLK_F20 as i32,
    F21 = sys::SDL_KeyCode::SDLK_F21 as i32,
    F22 = sys::SDL_KeyCode::SDLK_F22 as i32,
    F23 = sys::SDL_KeyCode::SDLK_F23 as i32,
    F24 = sys::SDL_KeyCode::SDLK_F24 as i32,
    Execute = sys::SDL_KeyCode::SDLK_EXECUTE as i32,
    Help = sys::SDL_KeyCode::SDLK_HELP as i32,
    Menu = sys::SDL_KeyCode::SDLK_MENU as i32,
    Select = sys::SDL_KeyCode::SDLK_SELECT as i32,
    Stop = sys::SDL_KeyCode::SDLK_STOP as i32,
    Again = sys::SDL_KeyCode::SDLK_AGAIN as i32,
    Undo = sys::SDL_KeyCode::SDLK_UNDO as i32,
    Cut = sys::SDL_KeyCode::SDLK_CUT as i32,
    Copy = sys::SDL_KeyCode::SDLK_COPY as i32,
    Paste = sys::SDL_KeyCode::SDLK_PASTE as i32,
    Find = sys::SDL_KeyCode::SDLK_FIND as i32,
    Mute = sys::SDL_KeyCode::SDLK_MUTE as i32,
    VolumeUp = sys::SDL_KeyCode::SDLK_VOLUMEUP as i32,
    VolumeDown = sys::SDL_KeyCode::SDLK_VOLUMEDOWN as i32,
    KpComma = sys::SDL_KeyCode::SDLK_KP_COMMA as i32,
    KpEqualsAS400 = sys::SDL_KeyCode::SDLK_KP_EQUALSAS400 as i32,
    AltErase = sys::SDL_KeyCode::SDLK_ALTERASE as i32,
    Sysreq = sys::SDL_KeyCode::SDLK_SYSREQ as i32,
    Cancel = sys::SDL_KeyCode::SDLK_CANCEL as i32,
    Clear = sys::SDL_KeyCode::SDLK_CLEAR as i32,
    Prior = sys::SDL_KeyCode::SDLK_PRIOR as i32,
    Return2 = sys::SDL_KeyCode::SDLK_RETURN2 as i32,
    Separator = sys::SDL_KeyCode::SDLK_SEPARATOR as i32,
    Out = sys::SDL_KeyCode::SDLK_OUT as i32,
    Oper = sys::SDL_KeyCode::SDLK_OPER as i32,
    ClearAgain = sys::SDL_KeyCode::SDLK_CLEARAGAIN as i32,
    CrSel = sys::SDL_KeyCode::SDLK_CRSEL as i32,
    ExSel = sys::SDL_KeyCode::SDLK_EXSEL as i32,
    Kp00 = sys::SDL_KeyCode::SDLK_KP_00 as i32,
    Kp000 = sys::SDL_KeyCode::SDLK_KP_000 as i32,
    ThousandsSeparator = sys::SDL_KeyCode::SDLK_THOUSANDSSEPARATOR as i32,
    DecimalSeparator = sys::SDL_KeyCode::SDLK_DECIMALSEPARATOR as i32,
    CurrencyUnit = sys::SDL_KeyCode::SDLK_CURRENCYUNIT as i32,
    CurrencySubUnit = sys::SDL_KeyCode::SDLK_CURRENCYSUBUNIT as i32,
    KpLeftParen = sys::SDL_KeyCode::SDLK_KP_LEFTPAREN as i32,
    KpRightParen = sys::SDL_KeyCode::SDLK_KP_RIGHTPAREN as i32,
    KpLeftBrace = sys::SDL_KeyCode::SDLK_KP_LEFTBRACE as i32,
    KpRightBrace = sys::SDL_KeyCode::SDLK_KP_RIGHTBRACE as i32,
    KpTab = sys::SDL_KeyCode::SDLK_KP_TAB as i32,
    KpBackspace = sys::SDL_KeyCode::SDLK_KP_BACKSPACE as i32,
    KpA = sys::SDL_KeyCode::SDLK_KP_A as i32,
    KpB = sys::SDL_KeyCode::SDLK_KP_B as i32,
    KpC = sys::SDL_KeyCode::SDLK_KP_C as i32,
    KpD = sys::SDL_KeyCode::SDLK_KP_D as i32,
    KpE = sys::SDL_KeyCode::SDLK_KP_E as i32,
    KpF = sys::SDL_KeyCode::SDLK_KP_F as i32,
    KpXor = sys::SDL_KeyCode::SDLK_KP_XOR as i32,
    KpPower = sys::SDL_KeyCode::SDLK_KP_POWER as i32,
    KpPercent = sys::SDL_KeyCode::SDLK_KP_PERCENT as i32,
    KpLess = sys::SDL_KeyCode::SDLK_KP_LESS as i32,
    KpGreater = sys::SDL_KeyCode::SDLK_KP_GREATER as i32,
    KpAmpersand = sys::SDL_KeyCode::SDLK_KP_AMPERSAND as i32,
    KpDblAmpersand = sys::SDL_KeyCode::SDLK_KP_DBLAMPERSAND as i32,
    KpVerticalBar = sys::SDL_KeyCode::SDLK_KP_VERTICALBAR as i32,
    KpDblVerticalBar = sys::SDL_KeyCode::SDLK_KP_DBLVERTICALBAR as i32,
    KpColon = sys::SDL_KeyCode::SDLK_KP_COLON as i32,
    KpHash = sys::SDL_KeyCode::SDLK_KP_HASH as i32,
    KpSpace = sys::SDL_KeyCode::SDLK_KP_SPACE as i32,
    KpAt = sys::SDL_KeyCode::SDLK_KP_AT as i32,
    KpExclam = sys::SDL_KeyCode::SDLK_KP_EXCLAM as i32,
    KpMemStore = sys::SDL_KeyCode::SDLK_KP_MEMSTORE as i32,
    KpMemRecall = sys::SDL_KeyCode::SDLK_KP_MEMRECALL as i32,
    KpMemClear = sys::SDL_KeyCode::SDLK_KP_MEMCLEAR as i32,
    KpMemAdd = sys::SDL_KeyCode::SDLK_KP_MEMADD as i32,
    KpMemSubtract = sys::SDL_KeyCode::SDLK_KP_MEMSUBTRACT as i32,
    KpMemMultiply = sys::SDL_KeyCode::SDLK_KP_MEMMULTIPLY as i32,
    KpMemDivide = sys::SDL_KeyCode::SDLK_KP_MEMDIVIDE as i32,
    KpPlusMinus = sys::SDL_KeyCode::SDLK_KP_PLUSMINUS as i32,
    KpClear = sys::SDL_KeyCode::SDLK_KP_CLEAR as i32,
    KpClearEntry = sys::SDL_KeyCode::SDLK_KP_CLEARENTRY as i32,
    KpBinary = sys::SDL_KeyCode::SDLK_KP_BINARY as i32,
    KpOctal = sys::SDL_KeyCode::SDLK_KP_OCTAL as i32,
    KpDecimal = sys::SDL_KeyCode::SDLK_KP_DECIMAL as i32,
    KpHexadecimal = sys::SDL_KeyCode::SDLK_KP_HEXADECIMAL as i32,
    LCtrl = sys::SDL_KeyCode::SDLK_LCTRL as i32,
    LShift = sys::SDL_KeyCode::SDLK_LSHIFT as i32,
    LAlt = sys::SDL_KeyCode::SDLK_LALT as i32,
    LGui = sys::SDL_KeyCode::SDLK_LGUI as i32,
    RCtrl = sys::SDL_KeyCode::SDLK_RCTRL as i32,
    RShift = sys::SDL_KeyCode::SDLK_RSHIFT as i32,
    RAlt = sys::SDL_KeyCode::SDLK_RALT as i32,
    RGui = sys::SDL_KeyCode::SDLK_RGUI as i32,
    Mode = sys::SDL_KeyCode::SDLK_MODE as i32,
    AudioNext = sys::SDL_KeyCode::SDLK_AUDIONEXT as i32,
    AudioPrev = sys::SDL_KeyCode::SDLK_AUDIOPREV as i32,
    AudioStop = sys::SDL_KeyCode::SDLK_AUDIOSTOP as i32,
    AudioPlay = sys::SDL_KeyCode::SDLK_AUDIOPLAY as i32,
    AudioMute = sys::SDL_KeyCode::SDLK_AUDIOMUTE as i32,
    MediaSelect = sys::SDL_KeyCode::SDLK_MEDIASELECT as i32,
    Www = sys::SDL_KeyCode::SDLK_WWW as i32,
    Mail = sys::SDL_KeyCode::SDLK_MAIL as i32,
    Calculator = sys::SDL_KeyCode::SDLK_CALCULATOR as i32,
    Computer = sys::SDL_KeyCode::SDLK_COMPUTER as i32,
    AcSearch = sys::SDL_KeyCode::SDLK_AC_SEARCH as i32,
    AcHome = sys::SDL_KeyCode::SDLK_AC_HOME as i32,
    AcBack = sys::SDL_KeyCode::SDLK_AC_BACK as i32,
    AcForward = sys::SDL_KeyCode::SDLK_AC_FORWARD as i32,
    AcStop = sys::SDL_KeyCode::SDLK_AC_STOP as i32,
    AcRefresh = sys::SDL_KeyCode::SDLK_AC_REFRESH as i32,
    AcBookmarks = sys::SDL_KeyCode::SDLK_AC_BOOKMARKS as i32,
    BrightnessDown = sys::SDL_KeyCode::SDLK_BRIGHTNESSDOWN as i32,
    BrightnessUp = sys::SDL_KeyCode::SDLK_BRIGHTNESSUP as i32,
    DisplaySwitch = sys::SDL_KeyCode::SDLK_DISPLAYSWITCH as i32,
    KbdIllumToggle = sys::SDL_KeyCode::SDLK_KBDILLUMTOGGLE as i32,
    KbdIllumDown = sys::SDL_KeyCode::SDLK_KBDILLUMDOWN as i32,
    KbdIllumUp = sys::SDL_KeyCode::SDLK_KBDILLUMUP as i32,
    Eject = sys::SDL_KeyCode::SDLK_EJECT as i32,
    Sleep = sys::SDL_KeyCode::SDLK_SLEEP as i32,
}

impl Keycode {
    pub fn from_i32(n: i32) -> Option<Keycode> {
        use self::Keycode::*;
        let n = n as u32;

        Some(match unsafe { transmute(n) } {
            sys::SDL_KeyCode::SDLK_BACKSPACE => Backspace,
            sys::SDL_KeyCode::SDLK_TAB => Tab,
            sys::SDL_KeyCode::SDLK_RETURN => Return,
            sys::SDL_KeyCode::SDLK_ESCAPE => Escape,
            sys::SDL_KeyCode::SDLK_SPACE => Space,
            sys::SDL_KeyCode::SDLK_EXCLAIM => Exclaim,
            sys::SDL_KeyCode::SDLK_QUOTEDBL => Quotedbl,
            sys::SDL_KeyCode::SDLK_HASH => Hash,
            sys::SDL_KeyCode::SDLK_DOLLAR => Dollar,
            sys::SDL_KeyCode::SDLK_PERCENT => Percent,
            sys::SDL_KeyCode::SDLK_AMPERSAND => Ampersand,
            sys::SDL_KeyCode::SDLK_QUOTE => Quote,
            sys::SDL_KeyCode::SDLK_LEFTPAREN => LeftParen,
            sys::SDL_KeyCode::SDLK_RIGHTPAREN => RightParen,
            sys::SDL_KeyCode::SDLK_ASTERISK => Asterisk,
            sys::SDL_KeyCode::SDLK_PLUS => Plus,
            sys::SDL_KeyCode::SDLK_COMMA => Comma,
            sys::SDL_KeyCode::SDLK_MINUS => Minus,
            sys::SDL_KeyCode::SDLK_PERIOD => Period,
            sys::SDL_KeyCode::SDLK_SLASH => Slash,
            sys::SDL_KeyCode::SDLK_0 => Num0,
            sys::SDL_KeyCode::SDLK_1 => Num1,
            sys::SDL_KeyCode::SDLK_2 => Num2,
            sys::SDL_KeyCode::SDLK_3 => Num3,
            sys::SDL_KeyCode::SDLK_4 => Num4,
            sys::SDL_KeyCode::SDLK_5 => Num5,
            sys::SDL_KeyCode::SDLK_6 => Num6,
            sys::SDL_KeyCode::SDLK_7 => Num7,
            sys::SDL_KeyCode::SDLK_8 => Num8,
            sys::SDL_KeyCode::SDLK_9 => Num9,
            sys::SDL_KeyCode::SDLK_COLON => Colon,
            sys::SDL_KeyCode::SDLK_SEMICOLON => Semicolon,
            sys::SDL_KeyCode::SDLK_LESS => Less,
            sys::SDL_KeyCode::SDLK_EQUALS => Equals,
            sys::SDL_KeyCode::SDLK_GREATER => Greater,
            sys::SDL_KeyCode::SDLK_QUESTION => Question,
            sys::SDL_KeyCode::SDLK_AT => At,
            sys::SDL_KeyCode::SDLK_LEFTBRACKET => LeftBracket,
            sys::SDL_KeyCode::SDLK_BACKSLASH => Backslash,
            sys::SDL_KeyCode::SDLK_RIGHTBRACKET => RightBracket,
            sys::SDL_KeyCode::SDLK_CARET => Caret,
            sys::SDL_KeyCode::SDLK_UNDERSCORE => Underscore,
            sys::SDL_KeyCode::SDLK_BACKQUOTE => Backquote,
            sys::SDL_KeyCode::SDLK_a => A,
            sys::SDL_KeyCode::SDLK_b => B,
            sys::SDL_KeyCode::SDLK_c => C,
            sys::SDL_KeyCode::SDLK_d => D,
            sys::SDL_KeyCode::SDLK_e => E,
            sys::SDL_KeyCode::SDLK_f => F,
            sys::SDL_KeyCode::SDLK_g => G,
            sys::SDL_KeyCode::SDLK_h => H,
            sys::SDL_KeyCode::SDLK_i => I,
            sys::SDL_KeyCode::SDLK_j => J,
            sys::SDL_KeyCode::SDLK_k => K,
            sys::SDL_KeyCode::SDLK_l => L,
            sys::SDL_KeyCode::SDLK_m => M,
            sys::SDL_KeyCode::SDLK_n => N,
            sys::SDL_KeyCode::SDLK_o => O,
            sys::SDL_KeyCode::SDLK_p => P,
            sys::SDL_KeyCode::SDLK_q => Q,
            sys::SDL_KeyCode::SDLK_r => R,
            sys::SDL_KeyCode::SDLK_s => S,
            sys::SDL_KeyCode::SDLK_t => T,
            sys::SDL_KeyCode::SDLK_u => U,
            sys::SDL_KeyCode::SDLK_v => V,
            sys::SDL_KeyCode::SDLK_w => W,
            sys::SDL_KeyCode::SDLK_x => X,
            sys::SDL_KeyCode::SDLK_y => Y,
            sys::SDL_KeyCode::SDLK_z => Z,
            sys::SDL_KeyCode::SDLK_DELETE => Delete,
            sys::SDL_KeyCode::SDLK_CAPSLOCK => CapsLock,
            sys::SDL_KeyCode::SDLK_F1 => F1,
            sys::SDL_KeyCode::SDLK_F2 => F2,
            sys::SDL_KeyCode::SDLK_F3 => F3,
            sys::SDL_KeyCode::SDLK_F4 => F4,
            sys::SDL_KeyCode::SDLK_F5 => F5,
            sys::SDL_KeyCode::SDLK_F6 => F6,
            sys::SDL_KeyCode::SDLK_F7 => F7,
            sys::SDL_KeyCode::SDLK_F8 => F8,
            sys::SDL_KeyCode::SDLK_F9 => F9,
            sys::SDL_KeyCode::SDLK_F10 => F10,
            sys::SDL_KeyCode::SDLK_F11 => F11,
            sys::SDL_KeyCode::SDLK_F12 => F12,
            sys::SDL_KeyCode::SDLK_PRINTSCREEN => PrintScreen,
            sys::SDL_KeyCode::SDLK_SCROLLLOCK => ScrollLock,
            sys::SDL_KeyCode::SDLK_PAUSE => Pause,
            sys::SDL_KeyCode::SDLK_INSERT => Insert,
            sys::SDL_KeyCode::SDLK_HOME => Home,
            sys::SDL_KeyCode::SDLK_PAGEUP => PageUp,
            sys::SDL_KeyCode::SDLK_END => End,
            sys::SDL_KeyCode::SDLK_PAGEDOWN => PageDown,
            sys::SDL_KeyCode::SDLK_RIGHT => Right,
            sys::SDL_KeyCode::SDLK_LEFT => Left,
            sys::SDL_KeyCode::SDLK_DOWN => Down,
            sys::SDL_KeyCode::SDLK_UP => Up,
            sys::SDL_KeyCode::SDLK_NUMLOCKCLEAR => NumLockClear,
            sys::SDL_KeyCode::SDLK_KP_DIVIDE => KpDivide,
            sys::SDL_KeyCode::SDLK_KP_MULTIPLY => KpMultiply,
            sys::SDL_KeyCode::SDLK_KP_MINUS => KpMinus,
            sys::SDL_KeyCode::SDLK_KP_PLUS => KpPlus,
            sys::SDL_KeyCode::SDLK_KP_ENTER => KpEnter,
            sys::SDL_KeyCode::SDLK_KP_1 => Kp1,
            sys::SDL_KeyCode::SDLK_KP_2 => Kp2,
            sys::SDL_KeyCode::SDLK_KP_3 => Kp3,
            sys::SDL_KeyCode::SDLK_KP_4 => Kp4,
            sys::SDL_KeyCode::SDLK_KP_5 => Kp5,
            sys::SDL_KeyCode::SDLK_KP_6 => Kp6,
            sys::SDL_KeyCode::SDLK_KP_7 => Kp7,
            sys::SDL_KeyCode::SDLK_KP_8 => Kp8,
            sys::SDL_KeyCode::SDLK_KP_9 => Kp9,
            sys::SDL_KeyCode::SDLK_KP_0 => Kp0,
            sys::SDL_KeyCode::SDLK_KP_PERIOD => KpPeriod,
            sys::SDL_KeyCode::SDLK_APPLICATION => Application,
            sys::SDL_KeyCode::SDLK_POWER => Power,
            sys::SDL_KeyCode::SDLK_KP_EQUALS => KpEquals,
            sys::SDL_KeyCode::SDLK_F13 => F13,
            sys::SDL_KeyCode::SDLK_F14 => F14,
            sys::SDL_KeyCode::SDLK_F15 => F15,
            sys::SDL_KeyCode::SDLK_F16 => F16,
            sys::SDL_KeyCode::SDLK_F17 => F17,
            sys::SDL_KeyCode::SDLK_F18 => F18,
            sys::SDL_KeyCode::SDLK_F19 => F19,
            sys::SDL_KeyCode::SDLK_F20 => F20,
            sys::SDL_KeyCode::SDLK_F21 => F21,
            sys::SDL_KeyCode::SDLK_F22 => F22,
            sys::SDL_KeyCode::SDLK_F23 => F23,
            sys::SDL_KeyCode::SDLK_F24 => F24,
            sys::SDL_KeyCode::SDLK_EXECUTE => Execute,
            sys::SDL_KeyCode::SDLK_HELP => Help,
            sys::SDL_KeyCode::SDLK_MENU => Menu,
            sys::SDL_KeyCode::SDLK_SELECT => Select,
            sys::SDL_KeyCode::SDLK_STOP => Stop,
            sys::SDL_KeyCode::SDLK_AGAIN => Again,
            sys::SDL_KeyCode::SDLK_UNDO => Undo,
            sys::SDL_KeyCode::SDLK_CUT => Cut,
            sys::SDL_KeyCode::SDLK_COPY => Copy,
            sys::SDL_KeyCode::SDLK_PASTE => Paste,
            sys::SDL_KeyCode::SDLK_FIND => Find,
            sys::SDL_KeyCode::SDLK_MUTE => Mute,
            sys::SDL_KeyCode::SDLK_VOLUMEUP => VolumeUp,
            sys::SDL_KeyCode::SDLK_VOLUMEDOWN => VolumeDown,
            sys::SDL_KeyCode::SDLK_KP_COMMA => KpComma,
            sys::SDL_KeyCode::SDLK_KP_EQUALSAS400 => KpEqualsAS400,
            sys::SDL_KeyCode::SDLK_ALTERASE => AltErase,
            sys::SDL_KeyCode::SDLK_SYSREQ => Sysreq,
            sys::SDL_KeyCode::SDLK_CANCEL => Cancel,
            sys::SDL_KeyCode::SDLK_CLEAR => Clear,
            sys::SDL_KeyCode::SDLK_PRIOR => Prior,
            sys::SDL_KeyCode::SDLK_RETURN2 => Return2,
            sys::SDL_KeyCode::SDLK_SEPARATOR => Separator,
            sys::SDL_KeyCode::SDLK_OUT => Out,
            sys::SDL_KeyCode::SDLK_OPER => Oper,
            sys::SDL_KeyCode::SDLK_CLEARAGAIN => ClearAgain,
            sys::SDL_KeyCode::SDLK_CRSEL => CrSel,
            sys::SDL_KeyCode::SDLK_EXSEL => ExSel,
            sys::SDL_KeyCode::SDLK_KP_00 => Kp00,
            sys::SDL_KeyCode::SDLK_KP_000 => Kp000,
            sys::SDL_KeyCode::SDLK_THOUSANDSSEPARATOR => ThousandsSeparator,
            sys::SDL_KeyCode::SDLK_DECIMALSEPARATOR => DecimalSeparator,
            sys::SDL_KeyCode::SDLK_CURRENCYUNIT => CurrencyUnit,
            sys::SDL_KeyCode::SDLK_CURRENCYSUBUNIT => CurrencySubUnit,
            sys::SDL_KeyCode::SDLK_KP_LEFTPAREN => KpLeftParen,
            sys::SDL_KeyCode::SDLK_KP_RIGHTPAREN => KpRightParen,
            sys::SDL_KeyCode::SDLK_KP_LEFTBRACE => KpLeftBrace,
            sys::SDL_KeyCode::SDLK_KP_RIGHTBRACE => KpRightBrace,
            sys::SDL_KeyCode::SDLK_KP_TAB => KpTab,
            sys::SDL_KeyCode::SDLK_KP_BACKSPACE => KpBackspace,
            sys::SDL_KeyCode::SDLK_KP_A => KpA,
            sys::SDL_KeyCode::SDLK_KP_B => KpB,
            sys::SDL_KeyCode::SDLK_KP_C => KpC,
            sys::SDL_KeyCode::SDLK_KP_D => KpD,
            sys::SDL_KeyCode::SDLK_KP_E => KpE,
            sys::SDL_KeyCode::SDLK_KP_F => KpF,
            sys::SDL_KeyCode::SDLK_KP_XOR => KpXor,
            sys::SDL_KeyCode::SDLK_KP_POWER => KpPower,
            sys::SDL_KeyCode::SDLK_KP_PERCENT => KpPercent,
            sys::SDL_KeyCode::SDLK_KP_LESS => KpLess,
            sys::SDL_KeyCode::SDLK_KP_GREATER => KpGreater,
            sys::SDL_KeyCode::SDLK_KP_AMPERSAND => KpAmpersand,
            sys::SDL_KeyCode::SDLK_KP_DBLAMPERSAND => KpDblAmpersand,
            sys::SDL_KeyCode::SDLK_KP_VERTICALBAR => KpVerticalBar,
            sys::SDL_KeyCode::SDLK_KP_DBLVERTICALBAR => KpDblVerticalBar,
            sys::SDL_KeyCode::SDLK_KP_COLON => KpColon,
            sys::SDL_KeyCode::SDLK_KP_HASH => KpHash,
            sys::SDL_KeyCode::SDLK_KP_SPACE => KpSpace,
            sys::SDL_KeyCode::SDLK_KP_AT => KpAt,
            sys::SDL_KeyCode::SDLK_KP_EXCLAM => KpExclam,
            sys::SDL_KeyCode::SDLK_KP_MEMSTORE => KpMemStore,
            sys::SDL_KeyCode::SDLK_KP_MEMRECALL => KpMemRecall,
            sys::SDL_KeyCode::SDLK_KP_MEMCLEAR => KpMemClear,
            sys::SDL_KeyCode::SDLK_KP_MEMADD => KpMemAdd,
            sys::SDL_KeyCode::SDLK_KP_MEMSUBTRACT => KpMemSubtract,
            sys::SDL_KeyCode::SDLK_KP_MEMMULTIPLY => KpMemMultiply,
            sys::SDL_KeyCode::SDLK_KP_MEMDIVIDE => KpMemDivide,
            sys::SDL_KeyCode::SDLK_KP_PLUSMINUS => KpPlusMinus,
            sys::SDL_KeyCode::SDLK_KP_CLEAR => KpClear,
            sys::SDL_KeyCode::SDLK_KP_CLEARENTRY => KpClearEntry,
            sys::SDL_KeyCode::SDLK_KP_BINARY => KpBinary,
            sys::SDL_KeyCode::SDLK_KP_OCTAL => KpOctal,
            sys::SDL_KeyCode::SDLK_KP_DECIMAL => KpDecimal,
            sys::SDL_KeyCode::SDLK_KP_HEXADECIMAL => KpHexadecimal,
            sys::SDL_KeyCode::SDLK_LCTRL => LCtrl,
            sys::SDL_KeyCode::SDLK_LSHIFT => LShift,
            sys::SDL_KeyCode::SDLK_LALT => LAlt,
            sys::SDL_KeyCode::SDLK_LGUI => LGui,
            sys::SDL_KeyCode::SDLK_RCTRL => RCtrl,
            sys::SDL_KeyCode::SDLK_RSHIFT => RShift,
            sys::SDL_KeyCode::SDLK_RALT => RAlt,
            sys::SDL_KeyCode::SDLK_RGUI => RGui,
            sys::SDL_KeyCode::SDLK_MODE => Mode,
            sys::SDL_KeyCode::SDLK_AUDIONEXT => AudioNext,
            sys::SDL_KeyCode::SDLK_AUDIOPREV => AudioPrev,
            sys::SDL_KeyCode::SDLK_AUDIOSTOP => AudioStop,
            sys::SDL_KeyCode::SDLK_AUDIOPLAY => AudioPlay,
            sys::SDL_KeyCode::SDLK_AUDIOMUTE => AudioMute,
            sys::SDL_KeyCode::SDLK_MEDIASELECT => MediaSelect,
            sys::SDL_KeyCode::SDLK_WWW => Www,
            sys::SDL_KeyCode::SDLK_MAIL => Mail,
            sys::SDL_KeyCode::SDLK_CALCULATOR => Calculator,
            sys::SDL_KeyCode::SDLK_COMPUTER => Computer,
            sys::SDL_KeyCode::SDLK_AC_SEARCH => AcSearch,
            sys::SDL_KeyCode::SDLK_AC_HOME => AcHome,
            sys::SDL_KeyCode::SDLK_AC_BACK => AcBack,
            sys::SDL_KeyCode::SDLK_AC_FORWARD => AcForward,
            sys::SDL_KeyCode::SDLK_AC_STOP => AcStop,
            sys::SDL_KeyCode::SDLK_AC_REFRESH => AcRefresh,
            sys::SDL_KeyCode::SDLK_AC_BOOKMARKS => AcBookmarks,
            sys::SDL_KeyCode::SDLK_BRIGHTNESSDOWN => BrightnessDown,
            sys::SDL_KeyCode::SDLK_BRIGHTNESSUP => BrightnessUp,
            sys::SDL_KeyCode::SDLK_DISPLAYSWITCH => DisplaySwitch,
            sys::SDL_KeyCode::SDLK_KBDILLUMTOGGLE => KbdIllumToggle,
            sys::SDL_KeyCode::SDLK_KBDILLUMDOWN => KbdIllumDown,
            sys::SDL_KeyCode::SDLK_KBDILLUMUP => KbdIllumUp,
            sys::SDL_KeyCode::SDLK_EJECT => Eject,
            sys::SDL_KeyCode::SDLK_SLEEP => Sleep,
            _ => return None,
        })
    }
}

use std::fmt;

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

use crate::keyboard::Scancode;

impl Keycode {
    /// Gets the virtual key from a scancode. Returns None if there is no corresponding virtual key.
    #[doc(alias = "SDL_GetKeyFromScancode")]
    pub fn from_scancode(scancode: Scancode) -> Option<Keycode> {
        const UNKNOWN: i32 = sys::SDL_KeyCode::SDLK_UNKNOWN as i32;
        unsafe {
            match sys::SDL_GetKeyFromScancode(transmute::<u32, sys::SDL_Scancode>(scancode as u32))
            {
                UNKNOWN => None,
                keycode_id => Keycode::from_i32(keycode_id as i32),
            }
        }
    }

    #[doc(alias = "SDL_GetKeyFromName")]
    pub fn from_name(name: &str) -> Option<Keycode> {
        const UNKNOWN: i32 = sys::SDL_KeyCode::SDLK_UNKNOWN as i32;
        unsafe {
            match CString::new(name) {
                Ok(name) => match sys::SDL_GetKeyFromName(name.as_ptr() as *const c_char) {
                    UNKNOWN => None,
                    keycode_id => Some(Keycode::from_i32(keycode_id as i32).unwrap()),
                },
                // string contains a nul byte - it won't match anything.
                Err(_) => None,
            }
        }
    }

    #[doc(alias = "SDL_GetKeyName")]
    pub fn name(self) -> String {
        // The name string pointer's contents _might_ change, depending on the last call to SDL_GetKeyName.
        // Knowing this, we must always return a new string.
        unsafe {
            let buf = sys::SDL_GetKeyName(self as i32);
            CStr::from_ptr(buf as *const _).to_str().unwrap().to_owned()
        }
    }
}
