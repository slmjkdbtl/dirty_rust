// wengwengweng

//! Input & Events

use std::path::PathBuf;
use std::collections::HashSet;

use once_cell::sync::Lazy;
pub use gilrs::GamepadId as GamepadID;

use crate::*;
use math::*;

pub type TouchID = u64;

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

impl Ctx {

	pub fn down_keys(&self) -> HashSet<Key> {

		use ButtonState::*;

		return self.key_states
			.iter()
			.filter(|(_, &state)| state == Down || state == Pressed)
			.map(|(key, _)| *key)
			.collect();

	}

	pub fn key_down(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Down) || self.key_pressed(key);
	}

	pub fn mouse_down(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Down) || self.mouse_pressed(mouse);
	}

	pub fn gamepad_down(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(states) = self.gamepad_button_states.get(&id) {
			return states.get(&button) == Some(&ButtonState::Down) || self.gamepad_pressed(id, button);
		} else {
			return false;
		}
	}

	pub fn key_mods(&self) -> KeyMod {
		return KeyMod {
			shift: self.key_down(Key::LShift) || self.key_down(Key::RShift),
			ctrl: self.key_down(Key::LCtrl) || self.key_down(Key::RCtrl),
			alt: self.key_down(Key::LAlt) || self.key_down(Key::RAlt),
			meta: self.key_down(Key::LMeta) || self.key_down(Key::RMeta),
		};
	}

	pub fn mouse_pos(&self) -> Vec2 {
		return self.mouse_pos;
	}

	pub fn key_pressed(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Pressed);
	}

	pub fn key_released(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Released);
	}

	pub fn key_up(&self, key: Key) -> bool {
		return self.key_states.get(&key) == Some(&ButtonState::Up) || self.key_states.get(&key).is_none();
	}

	pub fn mouse_pressed(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Pressed);
	}

	pub fn mouse_released(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Released);
	}

	pub fn mouse_up(&self, mouse: Mouse) -> bool {
		return self.mouse_states.get(&mouse) == Some(&ButtonState::Up) || self.mouse_states.get(&mouse).is_none();
	}

	pub fn gamepad_up(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(states) = self.gamepad_button_states.get(&id) {
			return states.get(&button) == Some(&ButtonState::Up) || states.get(&button).is_none();
		} else {
			return true;
		}
	}

	pub fn gamepad_pressed(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(states) = self.gamepad_button_states.get(&id) {
			return states.get(&button) == Some(&ButtonState::Pressed);
		} else {
			return false;
		}
	}

	pub fn gamepad_released(&self, id: GamepadID, button: GamepadButton) -> bool {
		if let Some(states) = self.gamepad_button_states.get(&id) {
			return states.get(&button) == Some(&ButtonState::Released);
		} else {
			return false;
		}
	}

}

#[derive(Clone, Debug)]
pub enum Event {
	KeyPress(Key),
	KeyPressRepeat(Key),
	KeyRelease(Key),
	MousePress(Mouse),
	MouseRelease(Mouse),
	MouseMove(Vec2),
	Scroll(Vec2, ScrollPhase),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ButtonState {
	Up,
	Pressed,
	Down,
	Released,
}

macro_rules! gen_buttons {

	// TODO: generate const str to replace $str
	($type:ident($xtype:ty), {$($name:ident($str:expr) => $xname:ident),*$(,)?}) => {

		#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
		pub enum $type {
			$(
				$name,
			)*
		}

		impl std::str::FromStr for $type {

			type Err = ();

			fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
				return match s {
					$(
						$str => Ok($type::$name),
					)*
					_ => Err(()),
				};
			}

		}

		impl $type {

			#[allow(dead_code)]
			#[cfg(desktop)]
			pub fn from_extern(s: $xtype) -> Option<Self> {
				return match s {
					$(
						<$xtype>::$xname => Some($type::$name),
					)*
					_ => None,
				};
			}

			// TODO
			#[allow(dead_code)]
			#[cfg(not(desktop))]
			pub fn from_extern(s: $xtype) -> Option<Self> {
				return None;
			}

			#[allow(dead_code)]
			pub fn as_str(&self) -> &'static str {
				return match self {
					$(
						$type::$name => $str,
					)*
				};
			}

			#[allow(dead_code)]
			#[cfg(desktop)]
			pub fn as_extern(&self) -> $xtype {
				return match self {
					$(
						$type::$name => <$xtype>::$xname,
					)*
				};
			}

		}

	}

}

#[cfg(not(web))]
use glutin::event::VirtualKeyCode as ExternKey;
#[cfg(not(web))]
use glutin::event::MouseButton as ExternMouse;
use gilrs::ev::Button as ExternGamepadButton;

#[cfg(web)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
	Esc,
	Left,
	Right,
	Back,
	RAlt,
	LAlt,
	RMeta,
	LMeta,
	RCtrl,
	LCtrl,
	RShift,
	LShift,
}

#[cfg(web)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mouse {
	Left,
	Right,
}

#[cfg(not(web))]
gen_buttons!(Key(ExternKey), {
	Q("q") => Q,
	W("w") => W,
	E("e") => E,
	R("r") => R,
	T("t") => T,
	Y("y") => Y,
	U("u") => U,
	I("i") => I,
	O("o") => O,
	P("p") => P,
	A("a") => A,
	S("s") => S,
	D("d") => D,
	F("f") => F,
	G("g") => G,
	H("h") => H,
	J("j") => J,
	K("k") => K,
	L("l") => L,
	Z("z") => Z,
	X("x") => X,
	C("c") => C,
	V("v") => V,
	B("b") => B,
	N("n") => N,
	M("m") => M,
	Key1("1") => Key1,
	Key2("2") => Key2,
	Key3("3") => Key3,
	Key4("4") => Key4,
	Key5("5") => Key5,
	Key6("6") => Key6,
	Key7("7") => Key7,
	Key8("8") => Key8,
	Key9("9") => Key9,
	Key0("0") => Key0,
	F1("f1") => F1,
	F2("f2") => F2,
	F3("f3") => F3,
	F4("f4") => F4,
	F5("f5") => F5,
	F6("f6") => F6,
	F7("f7") => F7,
	F8("f8") => F8,
	F9("f9") => F9,
	F10("f10") => F10,
	F11("f11") => F11,
	F12("f12") => F12,
	Minus("-") => Minus,
	Equals("=") => Equals,
	Comma(",") => Comma,
	Period(".") => Period,
	Grave("`") => Grave,
	Slash("/") => Slash,
	Backslash("\\") => Backslash,
	Semicolon(";") => Semicolon,
	Apostrophe("'") => Apostrophe,
	Up("up") => Up,
	Down("down") => Down,
	Left("left") => Left,
	Right("right") => Right,
	Esc("esc") => Escape,
	Tab("tab") => Tab,
	Space("space") => Space,
	Back("back") => Back,
	Return("return") => Return,
	LShift("lshift") => LShift,
	RShift("rshift") => RShift,
	LAlt("lalt") => LAlt,
	RAlt("ralt") => RAlt,
	LMeta("lmeta") => LWin,
	RMeta("rmeta") => RWin,
	LCtrl("lctrl") => LControl,
	RCtrl("rctrl") => RControl,
});

#[cfg(not(web))]
gen_buttons!(Mouse(ExternMouse), {
	Left("left") => Left,
	Right("right") => Right,
	Middle("middle") => Middle,
});

gen_buttons!(GamepadButton(ExternGamepadButton), {
	A("a") => South,
	B("b") => East,
	X("x") => West,
	Y("y") => North,
	LBumper("lbumper") => LeftTrigger,
	LTrigger("ltrigger") => LeftTrigger2,
	RBumper("rbumper") => RightTrigger,
	RTrigger("rtrigger") => RightTrigger2,
	Select("select") => Select,
	Start("start") => Start,
	Mode("mode") => Mode,
	LStick("lstick") => LeftThumb,
	RStick("rstick") => RightThumb,
	Up("up") => DPadUp,
	Down("down") => DPadDown,
	Left("left") => DPadLeft,
	Right("right") => DPadRight,
});

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

