use crate::lexer::TokenTree;

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

#[derive(Debug)]
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

#[derive(Debug)]
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
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Literal {
	String(String),
}

#[derive(Debug)]
pub enum Delimiter {
	CurlyBraces,
	Parentheses,
	SquareBrackets,
	AngleBrackets,
}
