use std::collections::VecDeque;
use std::ops::Index;

#[derive(Debug)]
pub struct Stack<T> {
	data: VecDeque<T>
}
impl<T> Stack<T> {
	pub fn new() -> Self {
		Self { data: VecDeque::new() }
	}
	pub fn from_vec(vec: Vec<T>) -> Self {
		let data = VecDeque::from(vec);
		Self { data }
	}

	pub fn push(&mut self, data: T) {
		self.data.push_back(data);
	}
	pub fn pop(&mut self) -> Option<T> {
		self.data.pop_back()
	}
	pub fn peek(&self) -> Option<&T> {
		self.data.get(0)
	}
	pub fn len(&self) -> usize {
		self.data.len()
	}
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
	pub fn get(&self, idx: usize) -> Option<&T> {
		self.data.get(idx)
	}
}

#[derive(Debug)]
pub struct Queue<T> {
	data: VecDeque<T>
}
impl<T> Queue<T> {
	pub fn new() -> Self {
		Self { data: VecDeque::new() }
	}
	pub fn from_vec(vec: Vec<T>) -> Self {
		let data = VecDeque::from(vec);
		Self { data }
	}

	pub fn enqueue(&mut self, data: T) {
		self.data.push_back(data);
	}
	pub fn dequeue(&mut self) -> Option<T> {
		self.data.pop_front()
	}
	pub fn len(&self) -> usize {
		self.data.len()
	}
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
	pub fn get(&self, idx: usize) -> Option<&T> {
		self.data.get(idx)
	}
}