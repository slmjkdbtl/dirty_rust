// wengwengweng

#[cfg(feature = "clip")]
use crate::clip;

use std::collections::HashSet;

pub type Line = i32;
pub type Col = i32;

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

impl Default for CursorPos {
	fn default() -> Self {
		return Self {
			line: 1,
			col: 1,
		};
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Range {
	pub start: CursorPos,
	pub end: CursorPos,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
	lines: Vec<String>,
	cursor: CursorPos,
	modified: bool,
}

#[derive(Clone)]
pub struct Conf {
	break_chars: HashSet<char>,
}

impl Default for Conf {
	fn default() -> Self {
		return Self {
			break_chars: hashset![
				' ',
				',',
				'.',
				';',
				':',
				'"',
				'(',
				')',
				'{',
				'}',
				'[',
				']',
				'<',
				'>',
				'_',
				'-',
				'@',
				'/',
				'\\',
				'\'',
				'\t'
			],
		};
	}
}

#[derive(Clone)]
pub struct TextBuffer {
	conf: Conf,
	lines: Vec<String>,
	cursor: CursorPos,
	modified: bool,
	undo_stack: Vec<State>,
	redo_stack: Vec<State>,
}

impl TextBuffer {

	pub fn new() -> Self {

		return Self {
			conf: Conf::default(),
			cursor: CursorPos::default(),
			lines: vec![String::new()],
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

			return self.cursor_bound(pos);

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

			return self.cursor_bound(pos);

		}

		return pos;

	}

	pub fn insert(&mut self, ch: char) {
		self.cursor = self.insert_at(self.cursor, ch);
	}

	/// delete secified line
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

	/// delete current line
	pub fn del_line(&mut self) {
		self.cursor.line = self.del_line_at(self.cursor.line);
	}

	/// get char at position
	pub fn char_at(&self, pos: CursorPos) -> Option<char> {
		return self.get_line_at(pos.line)?.chars().nth(pos.col as usize - 1);
	}

	/// break and insert new line, calculating indent
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

			return self.cursor_bound(pos);

		}

		return pos;

	}

	/// break_line_at() with cursor movement
	pub fn break_line(&mut self) {
		self.cursor = self.break_line_at(self.cursor);
	}

	/// delete char at specified position
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

	/// delete char at current cursor
	pub fn del(&mut self) {
		self.cursor = self.del_at(self.cursor);
	}

	/// delete the word at specified position
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
					return self.del_range(Range {
						start: prev_pos,
						end: CursorPos {
							col: pos.col - 1,
							.. pos
						},
					});
				}

			}

		}

		return pos;

	}

	/// delete the word before the cursor
	pub fn del_word(&mut self) {
		let pos = self.del_word_at(self.cursor);
		self.move_to(pos);
	}

	// TODO: multiline
	/// delete a range of text
	pub fn del_range(&mut self, r: Range) -> CursorPos {

		let start = r.start;
		let end = r.end;

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

	pub fn cursor_bound(&self, pos: CursorPos) -> CursorPos {

		if pos.col < 1 {
			return self.cursor_bound(CursorPos {
				col: 1,
				.. pos
			});
		}

		if pos.line < 1 {
			return self.cursor_bound(CursorPos {
				line: 1,
				.. pos
			});
		}

		if let Some(line) = self.get_line_at(pos.line) {

			let len = line.len() as Col + 1;

			if pos.col > len {

				return self.cursor_bound(CursorPos {
					col: len,
					.. pos
				});

			}

		}

		let lines = self.lines.len() as Line;

		if pos.line > lines && lines > 0 {
			return self.cursor_bound(CursorPos {
				line: lines,
				.. pos
			});
		}

		return pos;

	}

	pub fn move_to(&mut self, pos: CursorPos) {
		self.cursor = self.cursor_bound(pos);
	}

	/// move current cursor left
	pub fn move_left(&mut self) {
		self.move_to(CursorPos {
			col: self.cursor.col - 1,
			.. self.cursor
		});
	}

	/// move current cursor right
	pub fn move_right(&mut self) {
		self.move_to(CursorPos {
			col: self.cursor.col + 1,
			.. self.cursor
		});
	}

	/// move current cursor up
	pub fn move_up(&mut self) {
		self.move_to(CursorPos {
			line: self.cursor.line - 1,
			.. self.cursor
		});
	}

	/// move current cursor down
	pub fn move_down(&mut self) {
		self.move_to(CursorPos {
			line: self.cursor.line + 1,
			.. self.cursor
		});
	}

	/// move to the previous word
	pub fn move_prev_word(&mut self) {
		if let Some(pos) = self.prev_word() {
			self.move_to(pos);
		}
	}

	/// move to the next word
	pub fn move_next_word(&mut self) {
		if let Some(pos) = self.next_word() {
			self.move_to(pos);
		}
	}

	/// get next word position at specified position
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

	/// get next word position at current cursor
	pub fn next_word(&self) -> Option<CursorPos> {
		return self.next_word_at(self.cursor);
	}

	/// get previous word position at specified position
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

	/// get previous word position at current cursor
	pub fn prev_word(&self) -> Option<CursorPos> {
		return self.prev_word_at(self.cursor);
	}

	/// get the position that a line starts, ignoring tabs and spaces
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

			return self.cursor_bound(pos);

		}

		return pos;

	}

	/// line_start_at() with cursor movement
	pub fn move_line_start(&mut self) {
		self.cursor = self.line_start_at(self.cursor);
	}

	/// get the position that a line ends
	pub fn line_end_at(&self, mut pos: CursorPos) -> CursorPos {

		if let Some(line) = self.get_line_at(pos.line) {
			pos.col = line.len() as Col + 1;
			return self.cursor_bound(pos);
		}

		return pos;

	}

	/// line_end_at() with cursor movement
	pub fn move_line_end(&mut self) {
		self.cursor = self.line_end_at(self.cursor);
	}

	/// get current state for undo/redo
	fn get_state(&self) -> State {

		return State {
			lines: self.lines.clone(),
			cursor: self.cursor.clone(),
			modified: self.modified,
		};

	}

	/// set current state
	fn set_state(&mut self, state: State) {

		self.lines = state.lines;
		self.modified = state.modified;
		self.move_to(state.cursor);

	}

	/// push current state to undo stack
	pub fn push_undo(&mut self) {

		let state = self.get_state();

		if self.undo_stack.last() == Some(&state) {
			return;
		}

		self.undo_stack.push(state);

	}

	/// push current state to redo stack
	pub fn push_redo(&mut self) {
		self.redo_stack.push(self.get_state());
	}

	/// undo
	pub fn undo(&mut self) {

		if let Some(state) = self.undo_stack.pop() {
			self.push_redo();
			self.set_state(state);
		}

	}

	/// redo
	pub fn redo(&mut self) {

		if let Some(state) = self.redo_stack.pop() {
			self.push_undo();
			self.set_state(state);
		}

	}

	#[cfg(feature = "clip")]
	/// copy the whole specified line
	pub fn copy_line_at(&mut self, ln: Line) {
		if let Some(content) = self.get_line_at(ln).map(Clone::clone) {
			clip::set(content);
		}
	}

	#[cfg(feature = "clip")]
	/// copy current line
	pub fn copy_line(&mut self) {
		self.copy_line_at(self.cursor.line);
	}

	#[cfg(feature = "clip")]
	/// paste at specified pos
	pub fn paste_at(&mut self, pos: CursorPos) -> CursorPos {

		if let Ok(content) = clip::get() {
			return self.insert_str_at(pos, &content);
		}

		return pos;

	}

	#[cfg(feature = "clip")]
	/// paste at current cursor
	pub fn paste(&mut self) {
		self.cursor = self.paste_at(self.cursor);
	}

}

#[test]
fn buf_actions() {

	let mut buf = TextBuffer::new();

	buf.insert_str("1234567890");

	assert_eq!(buf.content(), String::from("1234567890"));
	assert_eq!(buf.cursor(), CursorPos::new(1, 11));

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

	assert_eq!(buf.content(), String::from("1234567 abc 890"));
	assert_eq!(buf.cursor(), CursorPos::new(1, 13));
	buf.move_prev_word();
	assert_eq!(buf.cursor(), CursorPos::new(1, 9));
	buf.move_prev_word();
	assert_eq!(buf.cursor(), CursorPos::new(1, 1));
	buf.move_next_word();
	assert_eq!(buf.cursor(), CursorPos::new(1, 8));
	buf.move_next_word();
	assert_eq!(buf.cursor(), CursorPos::new(1, 12));
	buf.move_next_word();
	assert_eq!(buf.cursor(), CursorPos::new(1, 16));
	buf.move_prev_word();
	assert_eq!(buf.cursor(), CursorPos::new(1, 13));

	buf.del();
	buf.del();

	assert_eq!(buf.content(), String::from("1234567 ab890"));
	assert_eq!(buf.cursor(), CursorPos::new(1, 11));

	buf.del_word();
	buf.move_left();

	assert_eq!(buf.content(), String::from("1234567 890"));
	assert_eq!(buf.cursor(), CursorPos::new(1, 8));

	buf.break_line();

	assert_eq!(buf.content(), String::from("1234567\n 890"));
	assert_eq!(buf.cursor(), CursorPos::new(2, 1));

	buf.move_line_end();
	assert_eq!(buf.cursor(), CursorPos::new(2, 5));
	buf.move_line_start();
	assert_eq!(buf.cursor(), CursorPos::new(2, 2));

}

