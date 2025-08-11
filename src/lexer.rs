use std::{error::Error, fmt::Display, mem::take, str::FromStr};

use tokens::Token;
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::lexer::{
	parse_literals::parse_literals,
	str_walker::StrWalker,
	tokens::{DELIMITER_MAP, TOKEN_MAP},
};

mod parse_literals;
mod str_walker;
pub mod tokens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErr {
	UnknownErr,
	UnbalancedNestingErr { start: String, end: String },
	ParseLiteralErr { typ: String, string: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenTree(Vec<Token>);

impl FromStr for TokenTree {
	type Err = LexErr;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		fn push_current_ident(tokens: &mut Vec<Token>, current_ident: &mut String) {
			if !current_ident.is_empty() {
				tokens.push(Token::Ident(take(current_ident)));
			}
		}

		let mut walker = StrWalker::new(input);
		let mut tokens = vec![];
		let mut current_ident = String::new();

		'lexing: while !walker.reached_end() {
			walker.jump_whitespace();

			// Comments
			if walker.get_between_recursive("<#", "#>").is_some() {
				push_current_ident(&mut tokens, &mut current_ident);
				continue;
			}
			if walker.currently_starts_with("#") {
				push_current_ident(&mut tokens, &mut current_ident);
				walker.jump_to_next("\n");
				continue;
			}

			// Blocks
			for (start, end, delimiter) in DELIMITER_MAP {
				let Some(str) = walker.get_between_recursive(start, end) else {
					continue;
				};

				push_current_ident(&mut tokens, &mut current_ident);
				tokens.push(Token::Block {
					delimiter: delimiter.clone(),
					tokentree: str?.parse()?,
				});

				continue 'lexing;
			}

			// Keywords/Symbols
			for (str, token) in TOKEN_MAP {
				if walker.currently_starts_with(str) {
					// Ensuring identifiers containing keywords, such as "inner" or "main", are properly parsed
					let keyword_valid = !current_ident.is_empty()
						|| walker.nth_after(str.len()).is_none_or(is_xid_continue);
					if let Token::Keyword(_) = token
						&& keyword_valid
					{
						continue;
					}

					push_current_ident(&mut tokens, &mut current_ident);
					walker.jump_by(str.len());
					tokens.push(token.clone());
					continue 'lexing;
				}
			}

			// Identifiers
			let Some(ch) = walker.next_char() else {
				continue;
			};
			if current_ident.is_empty() {
				if is_xid_start(ch) {
					current_ident.push(ch);
				} else {
					// Literals
					walker.jump_back(ch.len_utf8());
					tokens.push(Token::Literal(parse_literals(&mut walker)?));
				}
			} else if is_xid_continue(ch) {
				current_ident.push(ch);
			} else {
				walker.jump_back(ch.len_utf8());
				push_current_ident(&mut tokens, &mut current_ident);
			}
		}

		push_current_ident(&mut tokens, &mut current_ident);
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
	use crate::lexer::tokens::{Delimiter, Keyword, Literal, Symbol, Token};

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
				Token::Symbol(Symbol::Smaller),
				Token::Symbol(Symbol::Larger),
			]))
		)
	}

	#[test]
	fn comments() {
		let tt = "fun <# fun #>".parse::<TokenTree>();

		assert_eq!(tt, Ok(TokenTree(vec![Token::Keyword(Keyword::Fun)])))
	}

	#[test]
	fn literals() {
		let tt = "123456 123.456 true \"Hello, World!\" \'a\'".parse::<TokenTree>();

		assert_eq!(
			tt,
			Ok(TokenTree(vec![
				Token::Literal(Literal::Int(123456)),
				Token::Literal(Literal::Float(123.456)),
				Token::Literal(Literal::Bool(true)),
				Token::Literal(Literal::String("Hello, World!".into())),
				Token::Literal(Literal::Char('a'))
			]))
		)
	}

	#[test]
	fn ident() {
		let tt = "ident".parse::<TokenTree>();

		assert_eq!(tt, Ok(TokenTree(vec![Token::Ident("ident".to_string())])))
	}

	#[test]
	fn basic_fun() {
		let tt = "fun inner(param: String) => print(param)".parse::<TokenTree>();

		assert_eq!(
			tt,
			Ok(TokenTree(vec![
				Token::Keyword(Keyword::Fun),
				Token::Ident("inner".to_string()),
				Token::Block {
					delimiter: Delimiter::Parentheses,
					tokentree: TokenTree(vec![
						Token::Ident("param".to_string()),
						Token::Symbol(Symbol::Colon),
						Token::Ident("String".to_string()),
					])
				},
				Token::Symbol(Symbol::BigArrow),
				Token::Ident("print".to_string()),
				Token::Block {
					delimiter: Delimiter::Parentheses,
					tokentree: TokenTree(vec![Token::Ident("param".to_string())])
				},
			]))
		)
	}
}
