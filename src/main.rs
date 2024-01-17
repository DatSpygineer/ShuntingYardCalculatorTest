extern crate colored;

pub mod token;
mod collections;

use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, format, Formatter};
use std::io;
use std::io::{BufRead, Write};
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub};
use crate::token::*;
use crate::Value::{Float, Integer, Undefined};
use colored::Colorize;
use crate::collections::{Queue, Stack};

#[derive(Debug)]
pub enum Value {
	Undefined,
	Integer(i64),
	Float(f64)
}

impl Value {
	pub fn is_undefined(&self) -> bool {
		match self {
			Undefined => true,
			_ => false
		}
	}
	pub fn pow(&self, rhs: Value) -> Value {
		match self {
			Integer(i_left) => {
				match rhs {
					Integer(i_right) => {
						Integer(i_left.pow((i_right) as u32))
					},
					Float(f_right) => {
						Float((*i_left as f64).powf(f_right))
					}
					Undefined => Undefined
				}
			},
			Float(f_left) => {
				match rhs {
					Integer(i_right) => {
						Float(f_left.powi(i_right as i32))
					},
					Float(f_right) => {
						Float(f_left.powf(f_right))
					}
					Undefined => Undefined
				}
			}
			Undefined => Undefined
		}
	}
	pub fn floor(&self) -> Value {
		match self {
			Undefined => Undefined,
			Integer(int) => Integer(int.clone()),
			Float(flt) => Integer(flt.floor() as i64)
		}
	}
	pub fn as_float(&self) -> f64 {
		match self {
			Undefined => 0.0f64,
			Integer(int) => int.clone() as f64,
			Float(flt) => flt.clone()
		}
	}
}
impl Clone for Value {
	fn clone(&self) -> Self {
		match self {
			Undefined => Undefined,
			Integer(int) => Integer(int.clone()),
			Float(flt) => Float(flt.clone())
		}
	}
}

impl Neg for Value {
	type Output = Value;

	fn neg(self) -> Self::Output {
		match self {
			Undefined => Undefined,
			Integer(int) => { Integer(-int) },
			Float(flt) => { Float(-flt) }
		}
	}
}
impl Add for Value {
	type Output = Value;

	fn add(self, rhs: Self) -> Self::Output {
		match self {
			Integer(i_left) => {
				match rhs {
					Integer(i_right) => {
						Integer(i_left + i_right)
					},
					Float(f_right) => {
						Float(i_left as f64 + f_right)
					}
					Undefined => Undefined
				}
			},
			Float(f_left) => {
				match rhs {
					Integer(i_right) => {
						Float(f_left + (i_right as f64))
					},
					Float(f_right) => {
						Float(f_left + f_right)
					}
					Undefined => Undefined
				}
			}
			Undefined => Undefined
		}
	}
}
impl Sub for Value {
	type Output = Value;

	fn sub(self, rhs: Self) -> Self::Output {
		match self {
			Integer(i_left) => {
				match rhs {
					Integer(i_right) => {
						Integer(i_left - i_right)
					},
					Float(f_right) => {
						Float(i_left as f64 - f_right)
					}
					Undefined => Undefined
				}
			},
			Float(f_left) => {
				match rhs {
					Integer(i_right) => {
						Float(f_left - (i_right as f64))
					},
					Float(f_right) => {
						Float(f_left - f_right)
					}
					Undefined => Undefined
				}
			}
			Undefined => Undefined
		}
	}
}
impl Mul for Value {
	type Output = Value;

	fn mul(self, rhs: Self) -> Self::Output {
		match self {
			Integer(i_left) => {
				match rhs {
					Integer(i_right) => {
						Integer(i_left * i_right)
					},
					Float(f_right) => {
						Float(i_left as f64 * f_right)
					}
					Undefined => Undefined
				}
			},
			Float(f_left) => {
				match rhs {
					Integer(i_right) => {
						Float(f_left * (i_right as f64))
					},
					Float(f_right) => {
						Float(f_left * f_right)
					}
					Undefined => Undefined
				}
			}
			Undefined => Undefined
		}
	}
}
impl Div for Value {
	type Output = Value;

	fn div(self, rhs: Self) -> Self::Output {
		match self {
			Integer(i_left) => {
				match rhs {
					Integer(i_right) => {
						Integer(i_left / i_right)
					},
					Float(f_right) => {
						Float(i_left as f64 / f_right)
					}
					Undefined => Undefined
				}
			},
			Float(f_left) => {
				match rhs {
					Integer(i_right) => {
						Float(f_left / (i_right as f64))
					},
					Float(f_right) => {
						Float(f_left / f_right)
					}
					Undefined => Undefined
				}
			}
			Undefined => Undefined
		}
	}
}
impl Rem for Value {
	type Output = Value;

	fn rem(self, rhs: Self) -> Self::Output {
		match self {
			Integer(i_left) => {
				match rhs {
					Integer(i_right) => {
						Integer(i_left % i_right)
					},
					Float(f_right) => {
						Float((i_left as f64) % f_right)
					}
					Undefined => Undefined
				}
			},
			Float(f_left) => {
				match rhs {
					Integer(i_right) => {
						Float(f_left % (i_right as f64))
					},
					Float(f_right) => {
						Float(f_left % f_right)
					}
					Undefined => Undefined
				}
			}
			Undefined => Undefined
		}
	}
}
impl BitAnd for Value {
	type Output = Result<Value, String>;

	fn bitand(self, rhs: Self) -> Self::Output {
		match self {
			Integer(l_int) => {
				match rhs {
					Integer(r_int) => {
						Ok(Integer(l_int & r_int))
					},
					Float(_) => {
						Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::And))
					},
					Undefined => Ok(Undefined)
				}
			}
			Float(_) => {
				Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::And))
			}
			Undefined => Ok(Undefined)
		}
	}
}
impl BitOr for Value {
	type Output = Result<Value, String>;

	fn bitor(self, rhs: Self) -> Self::Output {
		match self {
			Integer(l_int) => {
				match rhs {
					Integer(r_int) => {
						Ok(Integer(l_int | r_int))
					},
					Float(_) => {
						Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Or))
					},
					Undefined => Ok(Undefined)
				}
			}
			Float(_) => {
				Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Or))
			}
			Undefined => Ok(Undefined)
		}
	}
}
impl BitXor for Value {
	type Output = Result<Value, String>;

	fn bitxor(self, rhs: Self) -> Self::Output {
		match self {
			Integer(l_int) => {
				match rhs {
					Integer(r_int) => {
						Ok(Integer(l_int ^ r_int))
					},
					Float(_) => {
						Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Xor))
					},
					Undefined => Ok(Undefined)
				}
			}
			Float(_) => {
				Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Xor))
			}
			Undefined => Ok(Undefined)
		}
	}
}
impl Shl for Value {
	type Output = Result<Value, String>;

	fn shl(self, rhs: Self) -> Self::Output {
		match self {
			Integer(l_int) => {
				match rhs {
					Integer(r_int) => {
						Ok(Integer(l_int << r_int))
					},
					Float(_) => {
						Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Shl))
					},
					Undefined => Ok(Undefined)
				}
			}
			Float(_) => {
				Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Shl))
			}
			Undefined => Ok(Undefined)
		}
	}
}
impl Shr for Value {
	type Output = Result<Value, String>;

	fn shr(self, rhs: Self) -> Self::Output {
		match self {
			Integer(l_int) => {
				match rhs {
					Integer(r_int) => {
						Ok(Integer(l_int << r_int))
					},
					Float(_) => {
						Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Shr))
					},
					Undefined => Ok(Undefined)
				}
			}
			Float(_) => {
				Err(format!("Failed to use operator '{:?}': Bitwise operators are not supported between floating-point values!", BinaryOperatorType::Shr))
			}
			Undefined => Ok(Undefined)
		}
	}
}

impl Eq for Value {}

impl PartialEq<Self> for Value {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Undefined => match other {
				Undefined => true,
				_ => false
			},
			Integer(l_int) => match other {
				Integer(r_int) => l_int == r_int,
				_ => false
			},
			Float(l_flt) => match other {
				Integer(r_int) => {
					*l_flt == (*r_int as f64)
				},
				Float(r_flt) => {
					l_flt == r_flt
				}
				Undefined => false
			}
		}
	}
}
impl PartialOrd<Self> for Value {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let left = match self {
			Integer(int) => int.clone() as f64,
			Float(flt) => flt.clone(),
			Undefined => f64::NAN
		};
		let right = match other {
			Integer(int) => int.clone() as f64,
			Float(flt) => flt.clone(),
			Undefined => f64::NAN
		};
		return left.partial_cmp(&right);
	}
}
impl Ord for Value {
	fn cmp(&self, other: &Self) -> Ordering {
		let left = match self {
			Integer(int) => int.clone() as f64,
			Float(flt) => flt.clone(),
			Undefined => f64::NAN
		};
		let right = match other {
			Integer(int) => int.clone() as f64,
			Float(flt) => flt.clone(),
			Undefined => f64::NAN
		};

		if left < right {
			Ordering::Less
		} else if left == right {
			Ordering::Equal
		} else {
			Ordering::Greater
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Undefined => write!(f, "undefined"),
			Integer(int) => write!(f, "{}", int),
			Float(flt) => write!(f, "{}", flt)
		}
	}
}

struct Calculator {
	globals: HashMap<String, Value>
}
impl Calculator {
	fn new() -> Self {
		Self {
			globals: HashMap::new()
		}
	}
	pub fn set_var(&mut self, name: &String, value: &Value) {
		if !self.globals.contains_key(name) {
			self.globals.insert(name.clone(), value.clone());
		} else {
			*self.globals.get_mut(name).unwrap() = value.clone();
		}
	}
	pub fn calculate(&mut self, src: String) -> Result<Value, String> {
		let tokens_r = Token::tokenize(src);
		if let Ok(tokens) = tokens_r {
			let mut operator_stack = Stack::new();
			let mut value_queue = Queue::new();

			println!("Tokens: {:?}", tokens);

			for token in &tokens {
				match token {
					Token::Integer(_) | Token::Float(_) | Token::Identifier(_) => {
						value_queue.enqueue(token);
					},
					Token::UnaryOperator(op) => {
						operator_stack.push(token);
					},
					Token::BinaryOperator(op) => {
						if let Some(top) = operator_stack.peek() {
							if let Token::BinaryOperator(mut op_other) = top {
								while op_other.order() > op.order() {
									value_queue.enqueue(operator_stack.pop().unwrap());
									if let Some(next) = operator_stack.peek() {
										if let Token::BinaryOperator(op_next) = next {
											op_other = *op_next;
										}
									} else {
										break;
									}
								}
							}
						}
						operator_stack.push(token);
					},
					Token::OpenParen => {
						operator_stack.push(token);
					},
					Token::CloseParen => {
						while let Some(op) = operator_stack.pop() {
							match op {
								Token::UnaryOperator(_) => { value_queue.enqueue(op) },
								Token::BinaryOperator(_) => { value_queue.enqueue(op) },
								Token::OpenParen => { break; },
								_ => { /* Do nothing */ }
							}
						}
					},
					_ => { return Err(format!("Unexpected token \"{:?}\"", token)); }
				}
			}

			while let Some(op) = operator_stack.pop() {
				value_queue.enqueue(op);
			}

			println!("Sorted values: {:?}", value_queue);

			let mut result_stack: Stack<Value> = Stack::new();

			while let Some(token) = value_queue.dequeue() {
				match token {
					Token::Integer(int) => {
						result_stack.push(Integer(*int));
					},
					Token::Float(flt) => {
						result_stack.push(Float(*flt));
					},
					Token::Identifier(symbol) => {
						if let Some(var) = self.globals.get(symbol) {
							result_stack.push(var.clone());
						} else {
							return Err(format!("Variable \"{}\" is undefined!", symbol));
						}
					},
					Token::UnaryOperator(op) => {
						if !result_stack.is_empty() {
							let value = result_stack.pop().unwrap();
							match op {
								UnaryOperatorType::Negative => {
									result_stack.push(-value);
								},
								UnaryOperatorType::Not => {
									result_stack.push(Integer(if value.as_float() == 0.0 { 1 } else { 0 }));
								},
								UnaryOperatorType::Invert => {
									match value {
										Integer(int) => {
											result_stack.push(Integer(!int));
										},
										_ => { return Err("Bitwise operations are only allowed for integer values!".to_string()); }
									}
								}
							}
						}
					},
					Token::BinaryOperator(op) => {
						if result_stack.len() >= 2 {
							let right = result_stack.pop().unwrap();
							let left = result_stack.pop().unwrap();
							match op {
								BinaryOperatorType::Add => {
									result_stack.push(left + right);
								},
								BinaryOperatorType::Sub => {
									result_stack.push(left - right);
								},
								BinaryOperatorType::Mul => {
									result_stack.push(left * right);
								},
								BinaryOperatorType::Div => {
									result_stack.push(left / right);
								},
								BinaryOperatorType::Mod => {
									result_stack.push(left % right);
								},
								BinaryOperatorType::Exp => {
									result_stack.push(left.pow(right));
								},
								BinaryOperatorType::Fdiv => {
									result_stack.push((left / right).floor());
								},
								BinaryOperatorType::And => {
									match left & right {
										Ok(result) => {
											result_stack.push(result);
										},
										Err(err) => {
											return Err(err);
										}
									}
								},
								BinaryOperatorType::Or => {
									match left | right {
										Ok(result) => {
											result_stack.push(result);
										},
										Err(err) => {
											return Err(err);
										}
									}
								},
								BinaryOperatorType::Xor => {
									match left ^ right {
										Ok(result) => {
											result_stack.push(result);
										},
										Err(err) => {
											return Err(err);
										}
									}
								},
								BinaryOperatorType::Shl => {
									match left << right {
										Ok(result) => {
											result_stack.push(result);
										},
										Err(err) => {
											return Err(err);
										}
									}
								},
								BinaryOperatorType::Shr => {
									match left >> right {
										Ok(result) => {
											result_stack.push(result);
										},
										Err(err) => {
											return Err(err);
										}
									}
								},
								BinaryOperatorType::Less => {
									result_stack.push(Integer(if left < right { 1 } else { 0 }));
								},
								BinaryOperatorType::LessEq => {
									result_stack.push(Integer(if left <= right { 1 } else { 0 }));
								},
								BinaryOperatorType::More => {
									result_stack.push(Integer(if left > right { 1 } else { 0 }));
								},
								BinaryOperatorType::MoreEq => {
									result_stack.push(Integer(if left >= right { 1 } else { 0 }));
								},
								BinaryOperatorType::Equal => {
									result_stack.push(Integer(if left == right { 1 } else { 0 }));
								},
								BinaryOperatorType::NotEqual => {
									result_stack.push(Integer(if left != right { 1 } else { 0 }));
								},
							}
						} else {
							return Err(format!("Failed to execute operation {:?}: Not enough values in result stack!", token));
						}
					},
					_ => { return Err(format!("Unexpected token \"{:?}\"", token)); }
				}
			}

			if let Some(result) = result_stack.pop() {
				Ok(result)
			} else {
				Err("No value in result stack!".to_string())
			}
		} else {
			Err(tokens_r.err().unwrap())
		}
	}
}

fn prompt(message: &'static str) -> String {
	print!("{}", message);
	io::stdout().flush().unwrap();
	let mut line = String::new();
	io::stdin().lock().read_line(&mut line).unwrap();
	return line.trim().to_string();
}

fn main() {
	let mut calc = Calculator::new();
	let mut line = prompt(">>> ");
	let mut assign = String::new();
	let mut display_as = NumberBaseType::Decimal;

	while line != "exit" {
		if line.starts_with("set ") {
			line = line.replace("set ", "");
			let c = line.chars().nth(0).unwrap();
			if !c.is_whitespace() && c.is_alphanumeric() || c == '_' {
				assign.push(c);
				line.remove(0);
			}
		} else if line.starts_with("hex ") {
			display_as = NumberBaseType::Hex;
			line = line.replace("hex ", "");
		} else if line.starts_with("dec ") {
			display_as = NumberBaseType::Decimal;
			line = line.replace("dec ", "");
		} else if line.starts_with("oct ") {
			display_as = NumberBaseType::Octal;
			line = line.replace("oct ", "");
		} else if line.starts_with("bin ") {
			display_as = NumberBaseType::Binary;
			line = line.replace("bin ", "");
		}

		match calc.calculate(line) {
			Ok(result) => {
				if assign.is_empty() {
					match display_as {
						NumberBaseType::Decimal => { println!("{}", result); },
						NumberBaseType::Binary => {
							match result {
								Undefined => { println!("undefined"); }
								Integer(int) => { println!("{:#b}", int); }
								Float(flt) => { println!("{}", flt); }
							}
						},
						NumberBaseType::Octal => {
							match result {
								Undefined => { println!("undefined"); }
								Integer(int) => { println!("{:#o}", int); }
								Float(flt) => { println!("{}", flt); }
							}
						},
						NumberBaseType::Hex => {
							match result {
								Undefined => { println!("undefined"); }
								Integer(int) => { println!("{:#X}", int); }
								Float(flt) => { println!("{}", flt); }
							}
						},
					}
				} else {
					calc.set_var(&assign, &result);
					println!("[{}]: {}", assign, result);
				}
			},
			Err(error) => {
				println!("{}", format!("Error: {}", error).red())
			}
		}

		assign.clear();
		display_as = NumberBaseType::Decimal;
		line = prompt(">>> ");
	}
}
