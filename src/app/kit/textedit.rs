// wengwengweng

use std::fmt;
use std::collections::HashSet;

use once_cell::sync::Lazy;

pub type Line = i32;
pub type Col = i32;

static BREAK_CHARS: Lazy<HashSet<char>> = Lazy::new(|| {
	return hset![' ', ',', '.', ';', ':', '"', '(', ')', '{', '}', '[', ']', '<', '>', '_', '-', '@', '/', '\\', '\'', '\t' ];
});

#[derive(Clone)]
pub struct Input {
	conf: Conf,
	content: String,
	cursor: Col,
	undo_stack: Vec<InputState>,
	redo_stack: Vec<InputState>,
}

#[derive(Debug, Clone, PartialEq)]
struct InputState {
	content: String,
	cursor: Col,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CursorDir {
	Start,
	End
}

#[derive(Clone)]
pub struct Conf {
	pub init_content: String,
	pub break_chars: HashSet<char>,
}

impl Default for Conf {
	fn default() -> Self {
		return Self {
			init_content: String::new(),
			break_chars: BREAK_CHARS.clone(),
		};
	}
}

fn is_private_use_char(c: char) -> bool {
	match c {
		'\u{E000}'..='\u{F8FF}' | '\u{F0000}'..='\u{FFFFD}' | '\u{100000}'..='\u{10FFFD}' => true,
		_ => false,
	}
}

impl Input {

	pub fn new() -> Self {
		return Self::with_conf(Conf::default());
	}

	pub fn with_conf(conf: Conf) -> Self {

		return Self {
			content: conf.init_content.clone(),
			conf: conf,
			cursor: 0,
			undo_stack: vec![],
			redo_stack: vec![],
		};

	}

	pub fn content(&self) -> &str {
		return &self.content;
	}

	pub fn set_content(&mut self, content: &str) {
		self.content = String::from(content);
	}

	pub fn cursor(&self) -> Col {
		return self.cursor;
	}

	fn clamp_cursor(&self, i: Col) -> Col {
		return i.clamp(0, self.content.len() as Col);
	}

	pub fn move_to(&mut self, i: Col) {
		self.cursor = self.clamp_cursor(i);
	}

	pub fn move_left(&mut self) {
		self.move_to(self.cursor - 1);
	}

	pub fn move_right(&mut self) {
		self.move_to(self.cursor + 1);
	}

	pub fn insert(&mut self, ch: char) {

		if ch.is_control() || is_private_use_char(ch) {
			return;
		}

		if self.conf.break_chars.contains(&ch) {
			self.push_undo();
		}

		self.content.insert(self.cursor as usize, ch);
		self.move_right();

	}

	pub fn insert_str(&mut self, s: &str) {

		self.content.insert_str(self.cursor as usize, s);
		self.move_to(self.cursor + s.len() as Col);

	}

	pub fn del(&mut self) {

		if self.content.is_empty() {
			return;
		}

		let ch = self.content.remove(self.cursor as usize - 1);
		self.move_left();

		if self.conf.break_chars.contains(&ch) {
			self.push_undo();
		}

	}

	fn get_prev_word(&self) -> Col {
		return get_prev_char(&self.content, self.cursor, |ch| {
			return self.conf.break_chars.contains(&ch);
		});
	}

	fn get_next_word(&self) -> Col {
		return get_next_char(&self.content, self.cursor, |ch| {
			return self.conf.break_chars.contains(&ch);
		});
	}

	pub fn move_prev_word(&mut self) {
		self.move_to(self.get_prev_word());
	}

	pub fn move_next_word(&mut self) {
		self.move_to(self.get_next_word());
	}

	pub fn del_word(&mut self) {

		let start = self.get_prev_word();

		self.del_range((start, self.cursor));
		self.move_to(start);

	}

	pub fn del_range(&mut self, range: (Col, Col)) {

		let (start, end) = range;
		let start = self.clamp_cursor(start);
		let end = self.clamp_cursor(end);

		self.push_undo();
		self.content.replace_range(start as usize..end as usize, "");

	}

	fn get_state(&self) -> InputState {

		return InputState {
			content: self.content.clone(),
			cursor: self.cursor,
		};

	}

	fn set_state(&mut self, state: InputState) {

		self.content = state.content;
		self.move_to(state.cursor);

	}

	pub fn push_undo(&mut self) {

		let state = self.get_state();

		if self.undo_stack.last() == Some(&state) {
			return;
		}

		self.undo_stack.push(state);

	}

	pub fn push_redo(&mut self) {
		self.redo_stack.push(self.get_state());
	}

	pub fn undo(&mut self) {

		if let Some(state) = self.undo_stack.pop() {

			self.push_redo();
			self.set_state(state);

		} else {

			self.set_state(InputState {
				content: self.conf.init_content.clone(),
				cursor: 0,
			});

		}

	}

	pub fn redo(&mut self) {
		if let Some(state) = self.redo_stack.pop() {
			self.push_undo();
			self.set_state(state);
		}
	}

}

fn get_prev_char(s: &str, c: Col, f: impl Fn(char) -> bool) -> Col {

	if let Some(chunk) = s.get(0..c as usize - 1) {
		for (i, ch) in chunk
			.char_indices()
			.rev()
			{
			if f(ch) {
				return i as Col + 1;
			}
		}
	}

	return 0;

}

fn get_next_char(s: &str, c: Col, f: impl Fn(char) -> bool) -> Col {

	let len = s.len();

	if let Some(chunk) = s.get(c as usize + 1..len) {
		for (i, ch) in chunk
			.char_indices()
			{
			if f(ch) {
				return i as Col + c + 1;
			}
		}
	}

	return s.len() as Col;

}

#[test]
fn input_actions() {

	let mut input = Input::new();

	let check = |b: &Input, content: &str, pos: Col| {

		let content1 = b.content();
		let content2 = content;
		let pos1 = b.cursor();
		let pos2 = pos;

		assert_eq!(content1, content2, "expected content '{}', found '{}'", content2, content1);
		assert_eq!(pos1, pos2, "expected cursor {}, found {}", pos2, pos1);

	};

	input.del();
	check(&input, "", 0);
	input.insert('a');
	check(&input, "a", 1);
	input.insert_str("bc");
	check(&input, "abc", 3);
	input.del();
	input.del();
	check(&input, "a", 1);
	input.insert_str("123 456");
	check(&input, "a123 456", 8);
	input.move_prev_word();
	check(&input, "a123 456", 5);
	input.move_prev_word();
	check(&input, "a123 456", 0);
	input.move_next_word();
	check(&input, "a123 456", 4);
	input.move_next_word();
	check(&input, "a123 456", 8);

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CursorPos {
	pub line: Line,
	pub col: Col,
}

impl CursorPos {
	pub fn new(l: Line, c: Col) -> Self {
		return Self {
			line: l,
			col: c,
		};
	}
}

impl fmt::Display for CursorPos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "({}, {})", self.line, self.col);
	}
}

impl Default for CursorPos {
	fn default() -> Self {
		return Self {
			line: 0,
			col: 0,
		};
	}
}

#[derive(Clone)]
pub struct TextArea {
	conf: Conf,
	lines: Vec<String>,
	cursor: CursorPos,
	modified: bool,
	undo_stack: Vec<TextAreaState>,
	redo_stack: Vec<TextAreaState>,
}

#[derive(Debug, Clone, PartialEq)]
struct TextAreaState {
	lines: Vec<String>,
	cursor: CursorPos,
	modified: bool,
}

impl TextArea {

	pub fn new() -> Self {
		return Self::with_conf(Conf::default());
	}

	pub fn with_conf(conf: Conf) -> Self {
		return Self {
			lines: conf.init_content.split('\n').map(String::from).collect(),
			conf: conf,
			cursor: CursorPos::default(),
			undo_stack: vec![],
			redo_stack: vec![],
			modified: false,
		};
	}

	/// set content
	pub fn set_content(&mut self, content: &str) {
		self.lines = content
			.split('\n')
			.map(|s| s.to_string())
			.collect();
	}

	/// get lines
	pub fn lines(&self) -> &[String] {
		return &self.lines;
	}

	/// get content as a String, lines joined with '\n'
	pub fn content(&self) -> String {
		return self.lines.join("\n");
	}

	/// get current cursor pos
	pub fn cursor(&self) -> CursorPos {
		return self.cursor;
	}

}

