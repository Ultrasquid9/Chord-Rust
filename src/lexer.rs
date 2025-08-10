use std::{error::Error, fmt::Display, str::FromStr};

use tokens::Token;

use crate::lexer::{
	str_walker::StrWalker,
	tokens::{DELIMITER_MAP, TOKEN_MAP},
};

mod str_walker;
pub mod tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErr {
	UnknownErr,
	UnbalancedNestingErr { start: String, end: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenTree(Vec<Token>);

impl FromStr for TokenTree {
	type Err = LexErr;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut walker = StrWalker::new(input);
		let mut tokens = vec![];

		'lexing: while !walker.reached_end() {
			walker.jump_whitespace();

			// Comments
			if walker.get_between_recursive("<#", "#>").is_some() {
				continue;
			}
			if walker.currently_starts_with("#") {
				walker.jump_to_next("\n");
				continue;
			}

			// Blocks
			for (start, end, delimiter) in DELIMITER_MAP {
				let Some(str) = walker.get_between_recursive(start, end) else {
					continue;
				};

				tokens.push(Token::Block {
					delimiter: delimiter.clone(),
					tokentree: str?.parse()?,
				});

				continue 'lexing;
			}

			// Tokens
			for (str, token) in TOKEN_MAP {
				if walker.currently_starts_with(str) {
					walker.jump_by(str.len());
					tokens.push(token.clone());
					continue 'lexing;
				}
			}

			walker.next_char();
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

impl Display for TokenTree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: proper display implementation
		f.write_str(&format!("{:?}", self.0))
	}
}

impl Error for LexErr {}

#[cfg(test)]
mod tests {
	use crate::lexer::tokens::{Delimiter, Keyword, Token};

	use super::TokenTree;

	#[test]
	fn blocks() {
		let tt = "{}[]()<>".parse::<TokenTree>();

		assert_eq!(
			tt,
			Ok(TokenTree(vec![
				Token::Block {
					delimiter: Delimiter::CurlyBraces,
					tokentree: TokenTree(vec![])
				},
				Token::Block {
					delimiter: Delimiter::SquareBrackets,
					tokentree: TokenTree(vec![])
				},
				Token::Block {
					delimiter: Delimiter::Parentheses,
					tokentree: TokenTree(vec![])
				},
				Token::Block {
					delimiter: Delimiter::AngleBrackets,
					tokentree: TokenTree(vec![])
				},
			]))
		)
	}

	#[test]
	fn comments() {
		let tt = "funct <# funct #>".parse::<TokenTree>();

		assert_eq!(tt, Ok(TokenTree(vec![Token::Keyword(Keyword::Funct)])))
	}
}
