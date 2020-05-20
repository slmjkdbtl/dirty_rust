// wengwengweng

use std::path::PathBuf;
use std::collections::HashSet;

use once_cell::sync::Lazy;
pub use gilrs::GamepadId as GamepadID;
pub type TouchID = u64;

use crate::*;
use math::*;

pub(crate) static INVALID_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
	return hset![
		// backspace
		'\u{7f}',
		// return
		'\r',
		'\n',
		// esc
		'\u{1b}',
		// unknown?
		'\u{8}',
		// up/down/left/right
		'\u{f700}',
		'\u{f701}',
		'\u{f702}',
		'\u{f703}',
		// f1 - f12
		'\u{f704}',
		'\u{f705}',
		'\u{f706}',
		'\u{f707}',
		'\u{f708}',
		'\u{f709}',
		'\u{f70a}',
		'\u{f70b}',
		'\u{f70c}',
		'\u{f70d}',
		'\u{f70e}',
		'\u{f70f}',
	];
});

#[derive(Clone, Copy, Debug)]
pub struct KeyMod {
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
	pub meta: bool,
}

#[derive(Clone, Debug)]
pub enum Event {
	KeyPress(Key),
	KeyPressRepeat(Key),
	KeyRelease(Key),
	MousePress(Mouse),
	MouseRelease(Mouse),
	MouseMove(Vec2),
	Wheel(Vec2, ScrollPhase),
	CharInput(char),
	GamepadPress(GamepadID, GamepadButton),
	GamepadPressRepeat(GamepadID, GamepadButton),
	GamepadRelease(GamepadID, GamepadButton),
	GamepadAxis(GamepadID, GamepadAxis, Vec2),
	GamepadConnect(GamepadID),
	GamepadDisconnect(GamepadID),
	Touch(TouchID, Vec2),
	Resize(i32, i32),
	FileHover(PathBuf),
	FileHoverCancel,
	FileDrop(PathBuf),
	Focus(bool),
	CursorEnter,
	CursorLeave,
	#[cfg(feature = "midi")]
	MIDI(midi::Msg),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
	Q,
	W,
	E,
	R,
	T,
	Y,
	U,
	I,
	O,
	P,
	A,
	S,
	D,
	F,
	G,
	H,
	J,
	K,
	L,
	Z,
	X,
	C,
	V,
	B,
	N,
	M,
	Key1,
	Key2,
	Key3,
	Key4,
	Key5,
	Key6,
	Key7,
	Key8,
	Key9,
	Key0,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	Minus,
	Equal,
	Comma,
	Period,
	Backquote,
	Slash,
	Backslash,
	Semicolon,
	Quote,
	Up,
	Down,
	Left,
	Right,
	Esc,
	Tab,
	Space,
	Backspace,
	Enter,
	LShift,
	RShift,
	LAlt,
	RAlt,
	LMeta,
	RMeta,
	LCtrl,
	RCtrl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mouse {
	Left,
	Right,
	Middle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GamepadButton {
	A,
	B,
	X,
	Y,
	LBumper,
	LTrigger,
	RBumper,
	RTrigger,
	Select,
	Start,
	Mode,
	LStick,
	RStick,
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug, Clone, Copy)]
pub enum ScrollPhase {
	Solid,
	Trailing,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GamepadAxis {
	LStick,
	RStick,
}

