trait AsLowerCase {
	fn as_lowercase(&self) -> char;
}
trait AsUpperCase {
	fn as_uppercase(&self) -> char;
}
impl AsLowerCase for char {
	fn as_lowercase(&self) -> char {
		let result: Vec<_> = self.to_lowercase().collect();
		return *result.first().unwrap();
	}
}
impl AsUpperCase for char {
	fn as_uppercase(&self) -> char {
		let result: Vec<_> = self.to_uppercase().collect();
		return *result.first().unwrap();
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum UnaryOperatorType {
	Negative,
	Not,
	Invert
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinaryOperatorType {
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Exp,
	Fdiv,
	And,
	Or,
	Xor,
	Less,
	LessEq,
	More,
	MoreEq,
	Equal,
	NotEqual
}
impl BinaryOperatorType {
	pub fn order(self) -> u32 {
		match self {
			BinaryOperatorType::Add => 4,
			BinaryOperatorType::Sub => 4,
			BinaryOperatorType::Mul => 5,
			BinaryOperatorType::Div => 5,
			BinaryOperatorType::Mod => 5,
			BinaryOperatorType::Exp => 6,
			BinaryOperatorType::Fdiv => 5,
			BinaryOperatorType::And => 2,
			BinaryOperatorType::Or => 0,
			BinaryOperatorType::Xor => 1,
			BinaryOperatorType::Less => 3,
			BinaryOperatorType::LessEq => 3,
			BinaryOperatorType::More => 3,
			BinaryOperatorType::MoreEq => 3,
			BinaryOperatorType::Equal => 3,
			BinaryOperatorType::NotEqual => 3
		}
	}
}

pub enum Token {
	Invalid,
	Integer(i64),
	Float(f64),
	Identifier(String),
	UnaryOperator(UnaryOperatorType),
	BinaryOperator(BinaryOperatorType),
	OpenParen,
	CloseParen
}

impl Clone for Token {
	fn clone(&self) -> Self {
		match self {
			Token::Invalid => Token::Invalid,
			Token::Integer(i) => Token::Integer(i.clone()),
			Token::Float(f) => Token::Float(f.clone()),
			Token::Identifier(id) => Token::Identifier(id.clone()),
			Token::UnaryOperator(u) => Token::UnaryOperator(u.clone()),
			Token::BinaryOperator(b) => Token::BinaryOperator(b.clone()),
			Token::OpenParen => Token::OpenParen,
			Token::CloseParen => Token::CloseParen
		}
	}
}

impl PartialEq<Self> for Token {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Token::Invalid => {
				match other {
					Token::Invalid => true,
					_ => false
				}
			},
			Token::Integer(_) => {
				match other {
					Token::Integer(_) => true,
					_ => false
				}
			},
			Token::Float(_) => {
				match other {
					Token::Float(_) => true,
					_ => false
				}
			},
			Token::Identifier(_) => {
				match other {
					Token::Identifier(_) => true,
					_ => false
				}
			},
			Token::UnaryOperator(_) => {
				match other {
					Token::UnaryOperator(_) => true,
					_ => false
				}
			},
			Token::BinaryOperator(_) => {
				match other {
					Token::BinaryOperator(_) => true,
					_ => false
				}
			},
			Token::OpenParen => {
				match other {
					Token::OpenParen => true,
					_ => false
				}
			},
			Token::CloseParen => {
				match other {
					Token::CloseParen => true,
					_ => false
				}
			}
		}
	}
}
impl Eq for Token {
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TokenizerState {
	Default,
	Number,
	Identifier,
	BinaryOperator
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NumberBaseType {
	Decimal,
	Binary,
	Octal,
	Hex
}
impl NumberBaseType {
	fn is_char_valid(self, c: char) -> bool {
		match self {
			NumberBaseType::Decimal => {
				c >= '0' && c <= '9'
			},
			NumberBaseType::Binary => {
				c == '0' || c == '1'
			},
			NumberBaseType::Octal => {
				c >= '0' && c <= '7'
			},
			NumberBaseType::Hex => {
				let clow = c.as_lowercase();
				(c >= '0' && c <= '9') || (clow >= 'a' && clow <= 'f')
			}
		}
	}
}

impl Token {
	pub fn is_operator(&self) -> bool {
		match self {
			Token::UnaryOperator(_) => true,
			Token::BinaryOperator(_) => true,
			_ => false
		}
	}
	pub fn is_unary_operator(&self) -> bool {
		match self {
			Token::UnaryOperator(_) => true,
			_ => false
		}
	}
	pub fn is_binary_operator(&self) -> bool {
		match self {
			Token::BinaryOperator(_) => true,
			_ => false
		}
	}
	fn char_is_operator(c: char) -> bool {
		"+-*/%&|^<>=".contains(c)
	}
	pub fn tokenize(src: &str) -> Result<Vec<Token>, String> {
		let mut tokens = Vec::new();
		let mut i = 0usize;
		let mut tokenValue = String::new();
		let mut foundDecimal = false;
		let mut numberBase = NumberBaseType::Decimal;
		let mut state = TokenizerState::Default;

		while i < src.len() {
			match src.chars().nth(i) {
				Some(mut c) => {
					match state {
						TokenizerState::Default => {
							if c.is_whitespace() {
								i += 1;
							} else if c.is_alphabetic() || c == '_' {
								state = TokenizerState::Identifier;
							} else if c.is_numeric() {
								if c == '0' {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_none() {
										tokens.push(Token::Integer(0));
										break;
									}
									c = cr.unwrap().as_lowercase();
									if c == 'x' {
										numberBase = NumberBaseType::Hex;
										i += 1;
									} else if c == 'b' {
										numberBase = NumberBaseType::Binary;
										i += 1;
									} else if c == 'o' || c == '0' {
										numberBase = NumberBaseType::Octal;
										i += 1;
									} else {
										numberBase = NumberBaseType::Decimal;
									}
								} else {
									numberBase = NumberBaseType::Decimal;
								}
								foundDecimal = false;
								tokenValue.clear();
								state = TokenizerState::Number;
							} else if "-!~".contains(c) {
								if c == '-' && (tokens.len() > 0 && !tokens.last().unwrap().is_binary_operator()) {
									state = TokenizerState::BinaryOperator;
								} else {
									match c {
										'-' => { tokens.push(Token::UnaryOperator(UnaryOperatorType::Negative)) },
										'!' => { tokens.push(Token::UnaryOperator(UnaryOperatorType::Not)) },
										'~' => { tokens.push(Token::UnaryOperator(UnaryOperatorType::Invert)) },
										_ => { return Err(format!("Invalid unary operator '{}'", c)) }
									}
									i += 1
								}
							} else if Token::char_is_operator(c) {
								state = TokenizerState::BinaryOperator;
							} else {
								return Err(format!("Unexpected character '{}'", c));
							}
						},
						TokenizerState::Number => {
							if c == '_' {
								i += 1;
							} else if numberBase == NumberBaseType::Decimal && c == '.' {
								if foundDecimal {
									return Err(format!("Invalid number literal \"{}{}\"", tokenValue, c));
								} else {
									tokenValue.push(c);
									foundDecimal = true;
									i += 1;
								}
							} else if numberBase.is_char_valid(c) {
								tokenValue.push(c);
								i += 1;
							} else if Token::char_is_operator(c) || c.is_whitespace() {
								match numberBase {
									NumberBaseType::Decimal => {
										if tokenValue.contains('.') {
											match tokenValue.parse::<f64>() {
												Ok(f) => {
													tokens.push(Token::Float(f))
												},
												Err(err) => {
													return Err(format!("Failed to parse number literal: \"{:?}\"", err));
												}
											}
										} else {
											match tokenValue.parse::<i64>() {
												Ok(i) => {
													tokens.push(Token::Integer(i))
												},
												Err(err) => {
													return Err(format!("Failed to parse number literal: \"{:?}\"", err));
												}
											}
										}
									}
									NumberBaseType::Binary => {
										match i64::from_str_radix(tokenValue.as_str(), 2) {
											Ok(i) => {
												tokens.push(Token::Integer(i))
											},
											Err(err) => {
												return Err(format!("Failed to parse number literal: \"{:?}\"", err));
											}
										}
									},
									NumberBaseType::Octal => {
										match i64::from_str_radix(tokenValue.as_str(), 8) {
											Ok(i) => {
												tokens.push(Token::Integer(i))
											},
											Err(err) => {
												return Err(format!("Failed to parse number literal: \"{:?}\"", err));
											}
										}
									},
									NumberBaseType::Hex => {
										match i64::from_str_radix(tokenValue.as_str(), 16) {
											Ok(i) => {
												tokens.push(Token::Integer(i))
											},
											Err(err) => {
												return Err(format!("Failed to parse number literal: \"{:?}\"", err));
											}
										}
									}
								}
								foundDecimal = false;
								tokenValue.clear();
								state = TokenizerState::Default;
							} else {
								return Err(format!("Invalid number literal \"{}{}\"", tokenValue, c));
							}
						},
						TokenizerState::Identifier => {
							if Token::char_is_operator(c) || c.is_whitespace() {
								tokens.push(Token::Identifier(tokenValue.clone()));
								tokenValue.clear();
								state = TokenizerState::Default;
							} else if c.is_alphanumeric() || c == '_' {
								tokenValue.push(c);
								i += 1;
							} else {
								return Err(format!("Unexpected character '{}'", c));
							}
						},
						TokenizerState::BinaryOperator => {}
					}
				},
				None => break
			}
		}

		return Ok(tokens);
	}
}