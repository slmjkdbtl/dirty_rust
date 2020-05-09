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

	if let Some(chunk) = s.get(0..(c - 1) as usize) {
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
			line: 1,
			col: 1,
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

	pub fn set_content(&mut self, content: &str) {
		self.lines = content
			.split('\n')
			.map(|s| s.to_string())
			.collect();
	}

	pub fn lines(&self) -> &[String] {
		return &self.lines;
	}

	pub fn content(&self) -> String {
		return self.lines.join("\n");
	}

	pub fn cursor(&self) -> CursorPos {
		return self.cursor;
	}

	pub fn get_line_at(&self, ln: Line) -> Option<&String> {
		return self.lines.get(ln as usize - 1);
	}

	pub fn get_line(&self) -> Option<&String> {
		return self.get_line_at(self.cursor.line);
	}

	pub fn set_line_at(&mut self, ln: Line, content: &str) {

		if self.get_line_at(ln).is_some() {

			if !self.modified {
				self.push_undo();
				self.redo_stack.clear();
				self.modified = true;
			}

			self.lines.get_mut(ln as usize - 1).map(|s| *s = String::from(content));

		}

	}

	pub fn set_line(&mut self, content: &str) {
		self.set_line_at(self.cursor.line, content);
	}

	pub fn insert_str_at(&mut self, mut pos: CursorPos, text: &str) -> CursorPos {

		if let Some(mut line) = self.get_line_at(pos.line).map(Clone::clone) {

			line.insert_str(pos.col as usize - 1, text);
			self.push_undo();
			self.set_line_at(pos.line, &line);
			pos.col += text.len() as Col;

			return self.clamp_cursor(pos);

		}

		return pos;

	}

	pub fn insert_str(&mut self, text: &str) {
		self.cursor = self.insert_str_at(self.cursor, text);
	}

	pub fn insert_at(&mut self, mut pos: CursorPos, ch: char) -> CursorPos {

		if !ch.is_ascii() {
			return pos;
		}

		if let Some(mut line) = self.get_line_at(pos.line).map(Clone::clone) {

			line.insert(pos.col as usize - 1, ch);

			if self.conf.break_chars.contains(&ch) {
				self.push_undo();
			}

			self.set_line_at(pos.line, &line);
			pos.col += 1;

			return self.clamp_cursor(pos);

		}

		return pos;

	}

	pub fn insert(&mut self, ch: char) {
		self.cursor = self.insert_at(self.cursor, ch);
	}

	pub fn del_line_at(&mut self, ln: Line) -> Line {

		if ln as usize <= self.lines.len() {

			self.push_undo();

			if !self.modified {
				self.redo_stack.clear();
				self.modified = true;
			}

			self.lines.remove(ln as usize - 1);

			if self.lines.is_empty() {
				self.lines = vec![String::from("")];
			}

		}

		return ln.clamp(1, self.lines.len() as Line);

	}

	pub fn del_line(&mut self) {
		self.cursor.line = self.del_line_at(self.cursor.line);
	}

	pub fn char_at(&self, pos: CursorPos) -> Option<char> {
		return self.get_line_at(pos.line)?.chars().nth(pos.col as usize - 1);
	}

	pub fn break_line_at(&mut self, mut pos: CursorPos) -> CursorPos {

		if let Some(line) = self.get_line_at(pos.line).map(Clone::clone) {

			let before = String::from(&line[0..pos.col as usize - 1]);
			let after = String::from(&line[pos.col as usize - 1..line.len()]);

			self.push_undo();

			if !self.modified {
				self.redo_stack.clear();
				self.modified = true;
			}

			self.lines.insert(pos.line as usize, String::new());
			self.set_line_at(pos.line, &before);
			self.set_line_at(pos.line + 1, &after);
			pos.line += 1;
			pos.col = 1;

			return self.clamp_cursor(pos);

		}

		return pos;

	}

	pub fn break_line(&mut self) {
		self.cursor = self.break_line_at(self.cursor);
	}

	pub fn del_at(&mut self, mut pos: CursorPos) -> CursorPos {

		if let Some(mut line) = self.get_line_at(pos.line).map(Clone::clone) {

			let before = &line[0..pos.col as usize - 1];

			if before.is_empty() {

				if let Some(mut prev_line) = self.get_line_at(pos.line - 1).map(Clone::clone) {

					let col = prev_line.len() as Col + 1;

					prev_line.push_str(&line);
					self.del_line_at(pos.line);
					self.set_line_at(pos.line - 1, &prev_line);
					pos.line -= 1;
					pos.col = col;

				}

			} else {

				line.remove(pos.col as usize - 2);
				self.set_line_at(pos.line, &line);
				pos.col -= 1;

			}

			return pos;

		}

		return pos;

	}

	pub fn del(&mut self) {
		self.cursor = self.del_at(self.cursor);
	}

	pub fn del_word_at(&mut self, mut pos: CursorPos) -> CursorPos {

		if let Some(line) = self.get_line_at(pos.line).map(Clone::clone) {

			let before = &line[0..pos.col as usize - 1];

			if before.is_empty() {

				if let Some(mut prev_line) = self.get_line_at(pos.line - 1).map(Clone::clone) {

					let col = prev_line.len() as Col + 1;

					prev_line.push_str(&line);
					self.del_line_at(pos.line);
					self.set_line_at(pos.line - 1, &prev_line);
					pos.line -= 1;
					pos.col = col;

				}

			} else {

				if let Some(prev_pos) = self.prev_word_at(pos) {
					return self.del_range((prev_pos, CursorPos {
						col: pos.col - 1,
						.. pos
					}));
				}

			}

		}

		return pos;

	}

	pub fn del_word(&mut self) {
		let pos = self.del_word_at(self.cursor);
		self.move_to(pos);
	}

	// TODO: multiline
	pub fn del_range(&mut self, r: (CursorPos, CursorPos)) -> CursorPos {

		let (start, end) = r;

		if start.line == end.line {

			if let Some(line) = self.get_line_at(start.line) {

				let mut line = line.clone();
				let start_col = (start.col - 1).clamp(0, line.len() as i32);
				let end_col = end.col.clamp(0, line.len() as i32);

				self.push_undo();
				line.replace_range(start_col as usize..end_col as usize, "");
				self.set_line_at(start.line, &line);

				return start;

			}

		}

		return self.cursor;

	}

	pub fn clamp_cursor(&self, pos: CursorPos) -> CursorPos {

		if pos.col < 1 {
			return self.clamp_cursor(CursorPos {
				col: 1,
				.. pos
			});
		}

		if pos.line < 1 {
			return self.clamp_cursor(CursorPos {
				line: 1,
				.. pos
			});
		}

		if let Some(line) = self.get_line_at(pos.line) {

			let len = line.len() as Col + 1;

			if pos.col > len {

				return self.clamp_cursor(CursorPos {
					col: len,
					.. pos
				});

			}

		}

		let lines = self.lines.len() as Line;

		if pos.line > lines && lines > 0 {
			return self.clamp_cursor(CursorPos {
				line: lines,
				.. pos
			});
		}

		return pos;

	}

	pub fn move_to(&mut self, pos: CursorPos) {
		self.cursor = self.clamp_cursor(pos);
	}

	pub fn move_left(&mut self) {
		self.move_to(CursorPos {
			col: self.cursor.col - 1,
			.. self.cursor
		});
	}

	pub fn move_right(&mut self) {
		self.move_to(CursorPos {
			col: self.cursor.col + 1,
			.. self.cursor
		});
	}

	pub fn move_up(&mut self) {
		self.move_to(CursorPos {
			line: self.cursor.line - 1,
			.. self.cursor
		});
	}

	pub fn move_down(&mut self) {
		self.move_to(CursorPos {
			line: self.cursor.line + 1,
			.. self.cursor
		});
	}

	pub fn move_prev_word(&mut self) {
		if let Some(pos) = self.prev_word() {
			self.move_to(pos);
		}
	}

	pub fn move_next_word(&mut self) {
		if let Some(pos) = self.next_word() {
			self.move_to(pos);
		}
	}

	pub fn next_word_at(&self, pos: CursorPos) -> Option<CursorPos> {

		let line = self.get_line_at(pos.line)?;

		if pos.col < line.len() as Col {

			for (i, ch) in line[pos.col as usize..].char_indices() {

				if self.conf.break_chars.contains(&ch) {
					return Some(CursorPos {
						col: pos.col + i as Col + 1 as Col,
						.. pos
					});
				}

			}

			return Some(CursorPos {
				col: line.len() as Col + 1,
				.. pos
			});

		}

		return None;

	}

	pub fn next_word(&self) -> Option<CursorPos> {
		return self.next_word_at(self.cursor);
	}

	pub fn prev_word_at(&self, pos: CursorPos) -> Option<CursorPos> {

		let line = self.get_line_at(pos.line)?;

		if pos.col <= line.len() as Col + 1 {

			let end = (pos.col - 2).clamp(0, line.len() as i32);

			for (i, ch) in line[..end as usize].char_indices().rev() {

				if self.conf.break_chars.contains(&ch) {
					return Some(CursorPos {
						col: i as Col + 2,
						.. pos
					});
				}

			}

			return Some(CursorPos {
				col: 1,
				.. pos
			});

		}

		return None;

	}

	pub fn prev_word(&self) -> Option<CursorPos> {
		return self.prev_word_at(self.cursor);
	}

	fn get_state(&self) -> TextAreaState {

		return TextAreaState {
			lines: self.lines.clone(),
			cursor: self.cursor.clone(),
			modified: self.modified,
		};

	}

	fn set_state(&mut self, state: TextAreaState) {

		self.lines = state.lines;
		self.modified = state.modified;
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
		}

	}

	pub fn redo(&mut self) {

		if let Some(state) = self.redo_stack.pop() {
			self.push_undo();
			self.set_state(state);
		}

	}

	pub fn line_start_at(&self, mut pos: CursorPos) -> CursorPos {

		if let Some(line) = self.get_line_at(pos.line) {

			let mut index = 0;

			for (i, ch) in line.chars().enumerate() {
				if ch != '\t' && ch != ' ' {
					index = i;
					break;
				} else if i == line.len() - 1 {
					index = i + 1;
				}
			}

			pos.col = index as Col + 1;

			return self.clamp_cursor(pos);

		}

		return pos;

	}

	pub fn move_line_start(&mut self) {
		self.cursor = self.line_start_at(self.cursor);
	}

	pub fn line_end_at(&self, mut pos: CursorPos) -> CursorPos {

		if let Some(line) = self.get_line_at(pos.line) {
			pos.col = line.len() as Col + 1;
			return self.clamp_cursor(pos);
		}

		return pos;

	}

	pub fn move_line_end(&mut self) {
		self.cursor = self.line_end_at(self.cursor);
	}

}

#[test]
fn textarea_actions() {

	let mut buf = TextArea::new();

	let check = |b: &TextArea, content: &str, l: Line, c: Col| {

		let content1 = b.content();
		let content2 = content;
		let pos1 = b.cursor();
		let pos2 = CursorPos::new(l, c);

		assert_eq!(content1, content2, "expected content '{}', found '{}'", content2, content1);
		assert_eq!(pos1, pos2, "expected cursor {}, found {}", pos2, pos1);

	};

	buf.insert_str("1234567890");

	check(&buf, "1234567890", 1, 11);

	buf.move_right();
	buf.move_up();
	buf.move_right();
	buf.move_left();
	buf.move_up();
	buf.move_left();
	buf.move_down();
	buf.move_left();

	buf.insert(' ');
	buf.insert_str("abc");
	buf.insert(' ');

	check(&buf, "1234567 abc 890", 1, 13);
	buf.move_prev_word();
	check(&buf, "1234567 abc 890", 1, 9);
	buf.move_prev_word();
	check(&buf, "1234567 abc 890", 1, 1);
	buf.move_next_word();
	check(&buf, "1234567 abc 890", 1, 8);
	buf.move_next_word();
	check(&buf, "1234567 abc 890", 1, 12);
	buf.move_next_word();
	check(&buf, "1234567 abc 890", 1, 16);
	buf.move_prev_word();
	check(&buf, "1234567 abc 890", 1, 13);

	buf.del();
	buf.del();

	check(&buf, "1234567 ab890", 1, 11);

	buf.del_word();
	buf.move_left();

	check(&buf, "1234567 890", 1, 8);

	buf.break_line();

	check(&buf, "1234567\n 890", 2, 1);

	buf.move_line_end();
	check(&buf, "1234567\n 890", 2, 5);
	buf.move_line_start();
	check(&buf, "1234567\n 890", 2, 2);

}

