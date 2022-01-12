pub enum Expression {
	Literal(Literal),
	Operation(Box<Expression>, Operator, Box<Expression>), // TODO: unary operators, operators with specific restrictions on operands, etc
}

pub struct Member {
	key: Identifier,
	value: Expression,
}

pub enum Literal {
	Boolean(bool),
	Number(f64),
	String(String),

	// TODO: if these contain expressions are they still literal? are they not literals and instead operators? currently they're both, which is definetly wrong.
	Tuple(Vec<Member>),
	Table(Vec<Member>),
	Function(Box<Deconstructor>, Box<Expression>),
}

pub enum Deconstructor {
	Literal(Literal),
	Wildcard,
	// tuples and tables are literals but they contain expressions which clearly don't work here
}

pub enum Operator {
	// TODO: better locations for this whole section
	DotIndex, // TODO: identifier for table / tuple or constant integer for tuple
	Index,
	Call,

	// TODO: unary
	Unary(UnaryOperator),

	Standard(StandardOperator),

	// TODO: left side deconstructors
	Assign(Option<StandardOperator>),
	Property, // TODO: only works in tuples and tables, left side identifier unless square brackets
	FunctionArrow,

	Semicolon,

	// NOTE: these are at the bottom because comma isn't actually an operator, it's part of these, and the location of this is the comma precedence
	Tuple,
	Table,
}
// comma is part of tuple and table

pub enum UnaryOperator {
	LogicalNot,
	BitwiseNot,
	Negative,
}

pub enum StandardOperator {
	Exponent, // TODO: better name, right to left associativity

	Multiply,
	Divide,
	Remainder,

	Add,
	Subtract,

	BitwiseLeftShift,
	BitwiseRightShift,
	BitwiseUnsignedRightShift,

	Less,
	LessEqual,
	Greater,
	GreaterEqual,

	Equal,
	NotEqual,

	BitwiseAnd,

	BitwiseXor,

	BitwiseOr,

	LogicalAnd,

	LogicalOr,

	If, // TODO: associativity, should if be here? if it is then if= would exist
	Else, // TODO: associativity
}