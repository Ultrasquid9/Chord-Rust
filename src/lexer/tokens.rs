use crate::lexer::TokenTree;

// Mildly hacky macro to reduce repetition in the token map
macro_rules! token_map {
	( $( $name:tt : $type:ident $token:ident ),+ $(,)? ) => {
		&[ $( ($name, Token::$type($type::$token) ), )+ ]
	};
}

pub const TOKEN_MAP: &[(&str, Token)] = token_map! [
	"funct": Keyword Funct,
	"struct": Keyword Struct,
	"enum": Keyword Enum,
	"alias": Keyword Alias,
	"trait": Keyword Trait,
	"extension": Keyword Extension,
	"impl": Keyword Impl,
	"using": Keyword Using,

	"var": Keyword Var,
	"let": Keyword Let,
	"const": Keyword Const,

	"if": Keyword If,
	"else": Keyword Else,
	"match": Keyword Match,

	"loop": Keyword Loop,
	"while": Keyword While,
	"for": Keyword For,
	"in": Keyword In,

	"break": Keyword Break,
	"continue": Keyword Continue,
	"return": Keyword Return,

	"mut": Keyword Mut,
	"fun": Keyword Fun,
	"try": Keyword Try,
	"yeet": Keyword Yeet,
	"unsafe": Keyword Unsafe,

	"->": Symbol Arrow,
	"=>": Symbol BigArrow,
	"|>": Symbol Pipe,

	"+": Symbol Add,
	"-": Symbol Subtract,
	"*": Symbol Multiply,
	"/": Symbol Divide,
	"%": Symbol Modulus,
	"!": Symbol Not,
	"&": Symbol And,
	"|": Symbol Or,
	"^": Symbol Xor,

	"==": Symbol Equal,
	"!=": Symbol NotEqual,
	">=": Symbol LargerOrEqual,
	"<=": Symbol SmallerOrEqual,
	">": Symbol Larger,
	"<": Symbol Smaller,

	"=": Symbol Assign,
	"+=": Symbol AddAssign,
	"-=": Symbol SubtractAssign,
	"*=": Symbol MultiplyAssign,
	"/=": Symbol DivideAssign,
	"%=": Symbol ModAssign,
	"&=": Symbol BitandAssign,
	"|=": Symbol BitorAssign,
	"^=": Symbol BitxorAssign,

	"~": Symbol Ref,
	"@": Symbol Deref,
];

pub const DELIMITER_MAP: &[(&str, &str, Delimiter)] = &[
	("{", "}", Delimiter::CurlyBraces),
	("(", ")", Delimiter::Parentheses),
	("[", "]", Delimiter::SquareBrackets),
	("<", ">", Delimiter::AngleBrackets),
];

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Keyword(Keyword),
	Symbol(Symbol),
	Literal(Literal),
	Ident(String),
	Block {
		delimiter: Delimiter,
		tokentree: TokenTree,
	},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
	Funct,
	Struct,
	Enum,
	Alias,
	Trait,
	Extension,
	Impl,
	Using,

	Var,
	Let,
	Const,

	If,
	Else,
	Match,

	Loop,
	While,
	For,
	In,

	Break,
	Continue,
	Return,

	// Reserved keywords, may get an official use eventually
	Mut,
	Fun,
	Try,
	Yeet,
	Unsafe,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
	Arrow,
	BigArrow,
	Pipe,
	Colon,

	Add,
	Subtract,
	Multiply,
	Divide,
	Modulus,
	Not,
	And,
	Or,
	Xor,

	Assign,
	AddAssign,
	SubtractAssign,
	MultiplyAssign,
	DivideAssign,
	ModAssign,
	BitandAssign,
	BitorAssign,
	BitxorAssign,

	Larger,
	Smaller,
	LargerOrEqual,
	SmallerOrEqual,
	Equal,
	NotEqual,

	Ref,
	Deref,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	String(String),
	Char(char),
	Bool(bool),
	Int(i64),
	Float(f64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
	CurlyBraces,
	Parentheses,
	SquareBrackets,
	AngleBrackets,
}
