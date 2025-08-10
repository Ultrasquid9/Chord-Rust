use crate::lexer::LexErr;

pub struct StrWalker<'input> {
	input: &'input str,
	index: usize,
}

impl<'input> StrWalker<'input> {
	pub fn new(input: &'input str) -> Self {
		Self { input, index: 0 }
	}

	/// Increases the internal index of the [StrWalker], returning the current [char] in the process.
	pub fn next_char(&mut self) -> Option<char> {
		if self.reached_end() {
			return None;
		}

		let og_index = self.index;
		self.index += 1;

		loop {
			if self.reached_end() || self.input.is_char_boundary(self.index) {
				return Some(
					self.input
						.get(og_index..self.index)?
						.chars()
						.next()
						.expect("Should be a valid char!"),
				);
			}

			self.index += 1;
		}
	}

	/// Whether or not the internal index has reached or passed the length of the [str].
	pub fn reached_end(&self) -> bool {
		self.index > self.input.len()
	}

	pub fn get_between_recursive(
		&mut self,
		start: &str,
		end: &str,
	) -> Option<Result<&'input str, LexErr>> {
		self.jump_whitespace();

		if self.currently_starts_with(start) {
			self.jump_by(start.len());
		} else {
			return None;
		}

		let og_index = self.index;
		let mut nesting: usize = 1;

		loop {
			if !self.input.is_char_boundary(self.index) {
				continue;
			}

			if self.currently_starts_with(start) {
				nesting += 1;
			} else if self.currently_starts_with(end) {
				nesting -= 1;

				let str = self.input.get(og_index..self.index)?;
				self.jump_by(end.len());

				if nesting == 0 {
					return Some(Ok(str));
				}
			}

			if self.reached_end() {
				return Some(Err(LexErr::UnbalancedNestingErr {
					start: start.to_string(),
					end: end.to_string(),
				}));
			}

			self.index += 1;
		}
	}

	/// Checks if the internal index is at the start of a pattern matching the target [str].
	pub fn currently_starts_with(&self, cmp: &str) -> bool {
		// Using wrapping_add, then checking the result, seems to be faster than
		// just using a normal add (likely because it doesn't panic)
		let end_index = self.index.wrapping_add(cmp.len());

		if end_index > self.input.len() || end_index < self.index {
			return false;
		}

		self.input.as_bytes()[self.index..end_index] == *cmp.as_bytes()
	}

	/// Jumps forwards by the provided amount of bytes.
	/// # Panics
	/// Panics if the index does not land on a valid char boundary.
	pub fn jump_by(&mut self, amount: usize) {
		self.index += amount;

		if !self.input.is_char_boundary(self.index) {
			panic!("{} is not a valid char boundary", self.index)
		}
	}

	/// Increases the internal index until it reaches a char that is not whitespace
	pub fn jump_whitespace(&mut self) {
		let mut og_index = self.index;

		loop {
			self.index += 1;

			if self.reached_end() {
				return;
			}

			let Some(c) = self.input.get(og_index..self.index) else {
				continue;
			};

			if c.chars().next().is_some_and(char::is_whitespace) {
				og_index = self.index;
			} else {
				self.index = og_index;
				return;
			}
		}
	}

	/// Jumps to the next instance of the target.
	pub fn jump_to_next(&mut self, target: &str) {
		loop {
			self.next_char();

			if self.currently_starts_with(target) || self.reached_end() {
				return;
			}
		}
	}
}
