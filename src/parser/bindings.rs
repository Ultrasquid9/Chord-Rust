use crate::{
	lexer::{
		TokenTree,
		tokens::{Keyword, Symbol, Token},
	},
	parser::ParseErr,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Binding {
	pub name: String,
	pub modifier: Option<BindingModifier>,
	pub typ: Option<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BindingModifier {
	Const,
	Var,
}

impl Binding {
	/// Parses a [`Binding`] from the start of the [`TokenTree`], consuming those tokens in the process.
	/// # Errors
	/// Returns a [`ParseErr`] if the [`TokenTree`] does not start with a valid binding.
	pub fn parse(tt: &mut TokenTree) -> Result<Binding, ParseErr> {
		let Some(token) = tt.pop() else {
			return Err(ParseErr::UnknownErr);
		};

		let mut name: Option<String> = None;
		let mut typ: Option<String> = None;

		let modifier = match token {
			Token::Keyword(Keyword::Const) => Some(BindingModifier::Const),
			Token::Keyword(Keyword::Var) => Some(BindingModifier::Var),

			Token::Ident(ident) => {
				name = Some(ident);
				None
			}
			_other => return Err(ParseErr::UnknownErr),
		};

		if name.is_none() {
			match tt.pop() {
				Some(Token::Ident(ident)) => {
					name = Some(ident);
				}
				_other => return Err(ParseErr::UnknownErr),
			}
		}

		if let Some(&Token::Symbol(Symbol::Colon)) = tt.first() {
			tt.pop();
			let Some(Token::Ident(ident)) = tt.pop() else {
				return Err(ParseErr::UnknownErr);
			};

			typ = Some(ident);
		}

		Ok(Binding {
			// SAFETY: `name` is known to exist, all other paths return
			name: unsafe { name.unwrap_unchecked() },
			modifier,
			typ,
		})
	}
}

/// Parses a list of arguments (struct fields, function args) from a [`TokenTree`].
/// # Errors
/// Returns a [`ParseErr`] if the [`TokenTree`] does not contain exclusively valid bindings.
pub fn parse_args(mut tt: TokenTree) -> Result<Vec<Binding>, ParseErr> {
	let mut args = vec![];

	loop {
		if tt.is_empty() {
			break;
		}

		args.push(Binding::parse(&mut tt)?);

		let Some(Token::Symbol(Symbol::Comma)) = tt.pop() else {
			break;
		};
	}

	Ok(args)
}
