struct AroundIter<T: Clone, I: Iterator<Item = T>> {
	iter: I,
	prev: Option<T>,
	cur: Option<T>,
	next: Option<T>,
}
impl<T: Clone, I: Iterator<Item = T>> AroundIter<T, I> {
	fn from(mut iter: I) -> Self {
		let next = iter.next();
		Self { iter, prev: None, cur: None, next }
	}
}
impl<T: Clone, I: Iterator<Item = T>> Iterator for AroundIter<T, I> {
	type Item = (Option<T>, T, Option<T>);
	fn next(&mut self) -> Option<(Option<T>, T, Option<T>)> {
		if self.next.is_some() {
			self.prev = self.cur.take();
			self.cur = self.next.take();
			self.next = self.iter.next();
		} else {
			return None;
		}
		Some((self.prev.take(), self.cur.clone().unwrap(), self.next.clone()))
	}
}

#[derive(Clone)]
struct Line {
	ints: Vec<(usize, usize, u32)>,
	syms: Vec<(usize, char)>
}

fn main() {
	let input = include_str!("../inputs/day3.txt");

	let around_iter = AroundIter::from(input.lines().map(|line| {
		let mut ints = vec![];
		let mut syms = vec![];
		let mut cur_int = None;
		for (col, c) in line.char_indices().chain([(line.len(), '\0')]) {
			if let Some(digit) = c.to_digit(10) {
				if let Some((_, end, val)) = &mut cur_int {
					*val = *val * 10 + digit;
					*end += 1;
				} else {
					cur_int = Some((col, col+1, digit));
				}
			} else {
				if let Some(int) = cur_int.take() {
					ints.push(int);
				}
				if c != '\0' && c != '.' {
					syms.push((col, c));
				}
			}
		}
		Line { ints, syms }
	}));
	
	let mut part_nb_sum = 0;
	let mut gear_ratio_sum = 0;
	for (prev, cur, next) in around_iter {
		let mut syms = cur.syms.clone();
		let mut ints = cur.ints.clone();
		if let Some(prev) = prev {
			syms.extend(prev.syms);
			ints.extend(prev.ints);
		}
		if let Some(next) = next {
			syms.extend(next.syms);
			ints.extend(next.ints);
		}

		for (start, end, val) in cur.ints {
			if syms.iter().any(|(pos,_)| pos+1 >= start && *pos < end+1) {
				part_nb_sum += val;
			}
		}

		for (pos, sym) in cur.syms {
			if sym == '*' {
				let part_nbs: Vec<u32> = ints.iter().copied()
					.filter_map(|(start, end, val)|
						if pos+1 >= start && pos < end+1 { Some(val) } else { None })
					.collect();
				if part_nbs.len() == 2 {
					gear_ratio_sum += part_nbs[0] * part_nbs[1];
				}
			}
		}
	}
	
	println!("part 1: {}", part_nb_sum);
	println!("part 2: {}", gear_ratio_sum);
}