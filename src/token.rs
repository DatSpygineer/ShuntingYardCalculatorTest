use crate::Value;

pub trait AsLowerCase {
	fn as_lowercase(&self) -> char;
}
pub trait AsUpperCase {
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
	Shl,
	Shr,
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
			BinaryOperatorType::Add => 5,
			BinaryOperatorType::Sub => 5,
			BinaryOperatorType::Mul => 6,
			BinaryOperatorType::Div => 6,
			BinaryOperatorType::Mod => 6,
			BinaryOperatorType::Exp => 7,
			BinaryOperatorType::Fdiv => 5,
			BinaryOperatorType::And => 2,
			BinaryOperatorType::Or => 0,
			BinaryOperatorType::Xor => 1,
			BinaryOperatorType::Shl => 4,
			BinaryOperatorType::Shr => 4,
			BinaryOperatorType::Less => 3,
			BinaryOperatorType::LessEq => 3,
			BinaryOperatorType::More => 3,
			BinaryOperatorType::MoreEq => 3,
			BinaryOperatorType::Equal => 3,
			BinaryOperatorType::NotEqual => 3
		}
	}
}

#[derive(Debug)]
pub enum Token {
	Invalid,
	Integer(i64),
	Float(f64),
	Identifier(String),
	UnaryOperator(UnaryOperatorType),
	BinaryOperator(BinaryOperatorType),
	Assignment,
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
			Token::Assignment => Token::Assignment,
			Token::OpenParen => Token::OpenParen,
			Token::CloseParen => Token::CloseParen,
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
			Token::Assignment => {
				match other {
					Token::Assignment => true,
					_ => false
				}
			}
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
	pub fn is_value(&self) -> bool {
		match self {
			Token::Integer(_) | Token::Float(_) | Token::Identifier(_) => true,
			_ => false
		}
	}
	pub fn is_identifier(&self) -> bool {
		match self {
			Token::Identifier(_) => true,
			_ => false
		}
	}
	pub fn as_value(&self) -> Option<Value> {
		match self {
			Token::Integer(int) => Some(Value::Integer(int.clone())),
			Token::Float(flt) => Some(Value::Float(flt.clone())),
			_ => None
		}
	}
	fn char_is_operator(c: char) -> bool {
		"+-*/%&|^<>=".contains(c)
	}
	pub fn tokenize(src: String) -> Result<Vec<Token>, String> {
		let mut tokens = Vec::new();
		let mut i = 0usize;
		let mut token_value = String::new();
		let mut found_decimal = false;
		let mut number_base = NumberBaseType::Decimal;
		let mut state = TokenizerState::Default;
		let mut should_parse = false;

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
										number_base = NumberBaseType::Hex;
										i += 1;
									} else if c == 'b' {
										number_base = NumberBaseType::Binary;
										i += 1;
									} else if c == 'o' || c.is_numeric() {
										number_base = NumberBaseType::Octal;
										if c == 'o' {
											i += 1;
										}
									} else {
										number_base = NumberBaseType::Decimal;
									}
								} else {
									number_base = NumberBaseType::Decimal;
								}
								found_decimal = false;
								token_value.clear();
								state = TokenizerState::Number;
							} else if "-!~".contains(c) {
								if c == '-' && (tokens.len() > 0 && !tokens.last().unwrap().is_binary_operator()) {
									state = TokenizerState::BinaryOperator;
								} else if c == '!' {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_none() {
										return Err(format!("Unexpected token {}", c));
									}

									c = cr.unwrap();
									if c == '=' {
										tokens.push(Token::BinaryOperator(BinaryOperatorType::NotEqual));
										i += 1;
									} else {
										tokens.push(Token::UnaryOperator(UnaryOperatorType::Not));
									}
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
							} else if c == '(' {
								tokens.push(Token::OpenParen);
								i += 1;
							} else if c == ')' {
								tokens.push(Token::CloseParen);
								i += 1;
							} else {
								return Err(format!("Unexpected character '{}'", c));
							}
						},
						TokenizerState::Number => {
							if c == '_' {
								i += 1;
							} else if number_base == NumberBaseType::Decimal && c == '.' {
								if found_decimal {
									return Err(format!("Invalid number literal \"{}{}\"", token_value, c));
								} else {
									token_value.push(c);
									found_decimal = true;
									i += 1;
								}
							} else if number_base.is_char_valid(c) {
								token_value.push(c);
								i += 1;
							} else if Token::char_is_operator(c) || c.is_whitespace() || c == '(' || c == ')' {
								should_parse = true;
							} else {
								return Err(format!("Invalid number literal \"{}{}\"", token_value, c));
							}

							if i + 1 >= src.len() || should_parse {
								match number_base {
									NumberBaseType::Decimal => {
										if token_value.contains('.') {
											match token_value.parse::<f64>() {
												Ok(f) => {
													tokens.push(Token::Float(f))
												},
												Err(err) => {
													return Err(format!("Failed to parse number literal: \"{:?}\"", err));
												}
											}
										} else {
											match token_value.parse::<i64>() {
												Ok(int) => {
													tokens.push(Token::Integer(int))
												},
												Err(err) => {
													return Err(format!("Failed to parse number literal: \"{:?}\"", err));
												}
											}
										}
									}
									NumberBaseType::Binary => {
										match i64::from_str_radix(token_value.as_str(), 2) {
											Ok(int) => {
												tokens.push(Token::Integer(int))
											},
											Err(err) => {
												return Err(format!("Failed to parse number literal: \"{:?}\"", err));
											}
										}
									},
									NumberBaseType::Octal => {
										match i64::from_str_radix(token_value.as_str(), 8) {
											Ok(int) => {
												tokens.push(Token::Integer(int))
											},
											Err(err) => {
												return Err(format!("Failed to parse number literal: \"{:?}\"", err));
											}
										}
									},
									NumberBaseType::Hex => {
										match i64::from_str_radix(token_value.as_str(), 16) {
											Ok(int) => {
												tokens.push(Token::Integer(int))
											},
											Err(err) => {
												return Err(format!("Failed to parse number literal: \"{:?}\"", err));
											}
										}
									}
								}
								should_parse = false;
								found_decimal = false;
								token_value.clear();
								state = TokenizerState::Default;
							}
						},
						TokenizerState::Identifier => {
							if Token::char_is_operator(c) || c.is_whitespace() || c == '(' || c == ')' || i + 1 >= src.len() {
								tokens.push(Token::Identifier(token_value.clone()));
								token_value.clear();
								state = TokenizerState::Default;
							} else if c.is_alphanumeric() || c == '_' {
								token_value.push(c);
								i += 1;
							} else {
								return Err(format!("Unexpected character '{}'", c));
							}
						},
						TokenizerState::BinaryOperator => {
							match c {
								'+' => {
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Add));
									state = TokenizerState::Default;
									i += 1;
								},
								'-' => {
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Sub));
									state = TokenizerState::Default;
									i += 1;
								},
								'*' => {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_some() {
										c = cr.unwrap();
										if c == '*' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::Exp));
											i += 1;
											state = TokenizerState::Default;
											continue;
										}
									}
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Mul));
									state = TokenizerState::Default;
								},
								'/' => {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_some() {
										c = cr.unwrap();
										if c == '/' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::Fdiv));
											i += 1;
											state = TokenizerState::Default;
											continue;
										}
									}
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Div));
									state = TokenizerState::Default;
								},
								'%' => {
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Mod));
									state = TokenizerState::Default;
									i += 1;
								},
								'&' => {
									tokens.push(Token::BinaryOperator(BinaryOperatorType::And));
									state = TokenizerState::Default;
									i += 1;
								},
								'|' => {
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Or));
									state = TokenizerState::Default;
									i += 1;
								},
								'^' => {
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Xor));
									state = TokenizerState::Default;
									i += 1;
								},
								'<' => {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_some() {
										c = cr.unwrap();
										if c == '<' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::Shl));
											i += 1;
											state = TokenizerState::Default;
											continue;
										} else if c == '=' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::LessEq));
											i += 1;
											state = TokenizerState::Default;
											continue;
										}
									}
									tokens.push(Token::BinaryOperator(BinaryOperatorType::Less));
									state = TokenizerState::Default;
								},
								'>' => {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_some() {
										c = cr.unwrap();
										if c == '>' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::Shr));
											i += 1;
											state = TokenizerState::Default;
											continue;
										} else if c == '=' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::MoreEq));
											i += 1;
											state = TokenizerState::Default;
											continue;
										}
									}
									tokens.push(Token::BinaryOperator(BinaryOperatorType::More));
									state = TokenizerState::Default;
								},
								'=' => {
									i += 1;
									let cr = src.chars().nth(i);
									if cr.is_some() {
										c = cr.unwrap();
										if c == '=' {
											tokens.push(Token::BinaryOperator(BinaryOperatorType::Equal));
											i += 1;
											state = TokenizerState::Default;
											continue;
										}
									}
									tokens.push(Token::Assignment);
									state = TokenizerState::Default;
								},
								_ => { return Err(format!("Invalid operator {}", c)) }
							}
						},
					}
				},
				None => break
			}

		}

		return Ok(tokens);
	}
}