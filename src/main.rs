use std::{error::Error, fmt::Debug};

use crate::lexer::TokenTree;

pub mod lexer;

/// Cannot be constructed, used as a placeholder
#[deprecated]
#[derive(Debug)]
pub enum TODO {}

const TEST: &str = include_str!("../test.ch");

fn main() -> Result<(), Box<dyn Error>> {
	let tt = TEST.parse::<TokenTree>()?;

	println!("{tt:?}");
	Ok(())
}
