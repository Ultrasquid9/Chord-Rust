use std::{error::Error, fmt::Display, str::FromStr};

use tokens::Token;

use crate::lexer::str_walker::StrWalker;

pub mod str_walker;
pub mod tokens;

#[derive(Debug)]
pub enum LexErr {
	UnknownErr,
	UnbalancedNestingErr { start: String, end: String },
}

#[derive(Debug)]
pub struct TokenTree(Vec<Token>);

impl FromStr for TokenTree {
	type Err = LexErr;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut walker = StrWalker::new(input);
		let mut tokens = vec![];

		while !walker.reached_end() {
			// Comments
			walker.get_between_recursive("<#", "#>");
			if walker.currently_starts_with("#") {
				walker.jump_to_next("\n");
			}
		}

		Ok(TokenTree(tokens))
	}
}

impl Display for LexErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: proper display implementation
		f.write_str(&format!("{self:?}"))
	}
}

impl Error for LexErr {}
