use unicode_ident::is_xid_continue;

use crate::lexer::{LexErr, str_walker::StrWalker, tokens::Literal};

pub fn parse_literals(walker: &mut StrWalker) -> Result<Literal, LexErr> {
	walker.jump_whitespace();

	// Strings
	if let Some(result) = walker.get_between_recursive("\"", "\"") {
		return Ok(Literal::String(result?.into()));
	}

	// Chars
	if let Some(result) = walker.get_between_recursive("\'", "\'") {
		let str = result?;

		match str.parse::<char>() {
			Ok(ch) => return Ok(Literal::Char(ch)),
			Err(_) => {
				return Err(LexErr::ParseLiteralErr {
					typ: "Char".into(),
					string: str.into(),
				});
			}
		}
	}

	// Booleans
	if walker.currently_starts_with("true") {
		walker.jump_by("true".len());
		return Ok(Literal::Bool(true));
	}
	if walker.currently_starts_with("false") {
		walker.jump_by("false".len());
		return Ok(Literal::Bool(false));
	}

	// Numbers
	parse_numbers(walker)
}

fn parse_numbers(walker: &mut StrWalker) -> Result<Literal, LexErr> {
	let mut bytes_after = 0;
	let mut numstr = String::new();

	loop {
		let Some(ch) = walker.nth_after(bytes_after) else {
			bytes_after += 1;
			continue;
		};

		if is_xid_continue(ch) || ch == '.' {
			numstr.push(ch);
			bytes_after += 1;
			continue;
		}

		if let Ok(num) = numstr.parse::<i64>() {
			walker.jump_by(numstr.len());
			return Ok(Literal::Int(num));
		}
		if let Ok(num) = numstr.parse::<f64>() {
			walker.jump_by(numstr.len());
			return Ok(Literal::Float(num));
		}

		return Err(LexErr::ParseLiteralErr {
			typ: "Number".into(),
			string: numstr,
		});
	}
}
