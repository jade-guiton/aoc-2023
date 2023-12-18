use std::{collections::HashMap, iter::Peekable};

use chumsky::prelude::*;

struct Step {
	dx: i32, dy: i32,
	steps: i32,
	color: u32,
}

fn int(radix: u32) -> impl Parser<char, String, Error=Simple<char>> {
	filter(move |c: &char| c.is_digit(radix)).repeated().collect()
}
fn parse_step() -> impl Parser<char, Step, Error=Simple<char>> {
	one_of("LRUD").then_ignore(just(' '))
		.then(int(10).from_str::<i32>().unwrapped()).then_ignore(just(" (#"))
		.then(int(16).map(|s: String| u32::from_str_radix(&s, 16)).unwrapped()).then_ignore(just(")"))
		.map(|((dir, steps), color)| Step {
			dx: match dir { 'L'=>-1, 'R'=>1, _=>0 },
			dy: match dir { 'U'=>-1, 'D'=>1, _=>0 },
			steps, color
		})
}

struct MergeIter<T: Ord, I1: Iterator<Item=T>, I2: Iterator<Item=T>> {
	left: Peekable<I1>,
	right: Peekable<I2>,
}
impl<T: Ord, I1: Iterator<Item=T>, I2: Iterator<Item=T>> MergeIter<T,I1,I2> {
	fn new(left: I1, right: I2) -> Self {
		MergeIter { left: left.peekable(), right: right.peekable() }
	}
}
impl<T: Ord, I1: Iterator<Item=T>, I2: Iterator<Item=T>> Iterator for MergeIter<T,I1,I2> {
	type Item = (bool, T);
	fn next(&mut self) -> Option<Self::Item> {
		if let Some(lhs) = self.left.peek() {
			if self.right.peek().map(|rhs| rhs < lhs).unwrap_or(false) {
				Some((true, self.right.next().unwrap()))
			} else {
				Some((false, self.left.next().unwrap()))
			}
		} else {
			self.right.next().map(|rhs| (true, rhs))
		}
	}
}

fn main() {
	let input = include_str!("../inputs/day18.txt");
	let mut plan = vec![];
	for line in input.lines() {
		match parse_step().parse(line) {
			Ok(step) => plan.push(step),
			Err(err) => {
				println!("{}", line);
				println!("{:?}", err);
				return;
			}
		}
	}
	
	let mut rows = HashMap::new();
	let mut x = 0;
	let mut y = 0;
	for step in plan {
		if step.dy == 0 {
			x += step.dx * step.steps;
		} else {
			for _ in 0..step.steps {
				rows.entry(y.min(y + step.dy)).or_insert_with(|| vec![]).push(x);
				y += step.dy;
			}
		}
	}
	assert!(x == 0 && y == 0);
	
	for row in rows.values_mut() {
		row.sort();
	}
	let min_y = *rows.keys().min().unwrap();
	let max_y = *rows.keys().max().unwrap() + 1;
	rows.insert(min_y-1, vec![]);
	rows.insert(max_y, vec![]);
	
	let mut area = 0;
	for y in min_y..=max_y {
		let top_row = rows.get(&(y-1)).unwrap();
		let bot_row = rows.get(&y).unwrap();
		let mut top_in = false;
		let mut bot_in = false;
		let mut run = None;
		println!("top: {:?}", top_row);
		println!("bot: {:?}", bot_row);
		for (side, x) in MergeIter::new(top_row.iter(), bot_row.iter()) {
			if side {
				bot_in = !bot_in;
			} else {
				top_in = !top_in;
			}
			let row_in = bot_in || top_in;
			if run.is_some() && !row_in {
				let x2 = run.take().unwrap();
				println!("{}: {} {}", y, x2, x);
				area += x - x2 + 1;
			} else if run.is_none() && row_in {
				run = Some(x);
			}
		}
		println!();
		assert!(run.is_none());
	}
	println!("part 1: {}", area);
}