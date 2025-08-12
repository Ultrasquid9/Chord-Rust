use std::{error::Error, fmt::Display};

use crate::lexer::TokenTree;

pub mod bindings;

#[derive(Clone, PartialEq, Debug)]
pub enum ParseErr {
	UnknownErr,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Ast;

impl Display for ParseErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: proper display implementation
		f.write_str(&format!("{self:?}"))
	}
}

impl Display for Ast {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: proper display implementation
		f.write_str(&format!("{self:?}"))
	}
}

impl Error for ParseErr {}

impl Ast {
	/// Parses the provided [`TokenTree`] into an [`Ast`].
	/// # Errors
	/// Returns a [`ParseErr`] if the [`TokenTree`] could not be parsed.
	pub fn parse(tt: TokenTree) -> Result<Self, ParseErr> {
		// TODO: parsing
		_ = tt;
		Err(ParseErr::UnknownErr)
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		lexer::{
			TokenTree,
			tokens::{Keyword, Symbol, Token},
		},
		parser::bindings::{Binding, BindingModifier, parse_args},
	};

	#[test]
	fn args() {
		let tt = TokenTree::new([
			Token::Keyword(Keyword::Const),
			Token::Ident("hello".into()),
			Token::Symbol(Symbol::Colon),
			Token::Ident("String".into()),
			Token::Symbol(Symbol::Comma),
			Token::Ident("world".into()),
			Token::Symbol(Symbol::Colon),
			Token::Ident("i32".into()),
			Token::Symbol(Symbol::Comma),
		]);

		assert_eq!(
			parse_args(tt),
			Ok(vec![
				Binding {
					name: "hello".into(),
					modifier: Some(BindingModifier::Const),
					typ: Some("String".into())
				},
				Binding {
					name: "world".into(),
					modifier: None,
					typ: Some("i32".into())
				},
			])
		)
	}
}
