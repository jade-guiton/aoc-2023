use std::fmt::Write;

#[derive(Clone, Copy)]
struct BitSet<const N: usize>([u64; N]);

impl<const N: usize> BitSet<N> {
	fn new() -> Self {
		Self([0; N])
	}
	fn set(&mut self, idx: usize) {
		self.0[idx / 64] |= 1u64 << (idx % 64);
	}
	fn check(&self, idx: usize) -> bool {
		self.0[idx / 64] >> (idx % 64) & 1 == 1
	}
	fn or(&self, other: Self) -> Self {
		let mut res = Self::new();
		for i in 0..N {
			res.0[i] = self.0[i] | other.0[i];
		}
		res
	}
}
impl<const N: usize> Default for BitSet<N> {
	fn default() -> Self {
		Self::new()
	}
}
impl<const N: usize> std::fmt::Debug for BitSet<N> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
			for i in 0..140 {
				f.write_char(if self.check(i) { '#' } else { '.' })?;
			}
			Ok(())
	}
}

struct AroundIter<T: Default + Clone, I: Iterator<Item = T>> {
	iter: I,
	prev: Option<T>,
	cur: Option<T>,
}
impl<T: Default + Clone, I: Iterator<Item = T>> AroundIter<T, I> {
	fn from(mut iter: I) -> Self {
		let cur = iter.next();
		Self { iter, prev: None, cur }
	}
}
impl<T: Default + Clone, I: Iterator<Item = T>> Iterator for AroundIter<T, I> {
	type Item = (Option<T>, T, Option<T>);
	fn next(&mut self) -> Option<(Option<T>, T, Option<T>)> {
		if let Some(cur) = self.cur.clone() {
			let next = self.iter.next();
			let res = (self.prev.take(), cur, next.clone());
			self.prev = self.cur.take();
			self.cur = next;
			Some(res)
		} else {
			return None;
		}
	}
}

fn main() {
	let input = include_str!("../inputs/day3.txt");

	// iterator returning set of symbol indices for each line
	let sym_set_iter = AroundIter::from(input.lines().map(|line| {
		let mut syms = BitSet::<3>::new();
		for (i, c) in line.char_indices() {
			if !c.is_ascii_digit() && c != '.' {
				if i > 0 { syms.set(i-1); }
				syms.set(i);
				if i < line.len()-1 { syms.set(i+1); }
			}
		}
		syms
	})).map(|(prev, cur, next)| cur.or(prev.unwrap_or_default()).or(next.unwrap_or_default()));

	let mut part_nb_sum = 0;
	for ((line_idx, line), sym_set) in input.lines().enumerate().zip(sym_set_iter) {
		println!("{}", line);
		println!("{:?}", sym_set);
		let mut cur_int = None;
		let mut is_part_nb = false;
		let mut it = line.char_indices();
		loop {
			let opt_c = it.next();
			if let Some((i, d)) = opt_c.clone().and_then(|(i,c)| c.to_digit(10).map(|d| (i,d))) {
				if let Some(int_val) = &mut cur_int {
					*int_val = *int_val * 10 + d;
				} else {
					cur_int = Some(d);
				}
				if sym_set.check(i) {
					is_part_nb = true;
				}
			} else if let Some(int_val) = cur_int {
				if is_part_nb {
					println!("line {}: part {}", line_idx, int_val);
					part_nb_sum += int_val;
					is_part_nb = false;
				}
				cur_int = None;
			}
			if opt_c.is_none() { break; }
		}
		//if line_idx == 3 { break; }
	}

	println!("part 1: {}", part_nb_sum);
}