use std::ops::RangeInclusive;

use chumsky::prelude::*;

type Ran = RangeInclusive<u32>;

fn intersect(r1: Ran, r2: Ran) -> Option<(Option<Ran>, Ran, Option<Ran>)> {
	if r1.start() <= r2.end() && r1.end() >= r2.start() {
		let inner_start = r1.start().max(r2.start());
		let inner_end = r1.end().min(r2.end());
		let mut before = None;
		if r1.start() < inner_start {
			before = Some(*r1.start() ..= inner_start-1);
		}
		let mut after = None;
		if r1.end() > inner_end {
			after = Some(inner_end+1 ..= *r1.end());
		}
		Some((before, *inner_start ..= *inner_end, after))
	} else {
		None
	}
}

struct Map {
	ranges: Vec<(u32, u32, u32)>,
}
impl Map {
	fn map(&self, x: u32) -> u32 {
		for (dst, src, len) in &self.ranges {
			if x >= *src && x <= src + (len-1) {
				return dst + (x - src);
			}
		}
		x
	}
	fn map_range(&self, ran: Ran) -> Vec<Ran> {
		let mut dst_ranges = vec![];
		let mut src_ranges = vec![ran];
		for (dst, src, len) in &self.ranges {
			let end = src + (len - 1);
			let mut new_ranges = vec![];
			src_ranges.retain(|ran| {
				if let Some((before, inner, after)) = intersect(ran.clone(), *src..=end) {
					if let Some(before) = before {
						new_ranges.push(before);
					}
					if let Some(after) = after {
						new_ranges.push(after);
					}
					dst_ranges.push(dst+(inner.start()-src) ..= dst+(inner.end()-src));
					false
				} else {
					true
				}
			});
			src_ranges.extend(new_ranges);
		}
		dst_ranges.extend(src_ranges);
		dst_ranges
	}
}

fn input_parser() -> impl Parser<char, (Vec<u32>, Vec<Map>), Error=Simple<char>> {
	just("seeds: ").ignore_then(text::int(10).from_str().unwrapped().separated_by(just(" ")))
		.then_ignore(just("\n\n"))
		.then((
			text::ident().then(just("-to-")).then(text::ident()).then(just(" map:\n")).ignore_then(
				text::int(10).from_str().unwrapped().separated_by(just(" ")).exactly(3).map(|v| (v[0], v[1], v[2]))
					.separated_by(just("\n")).map(|ranges| Map { ranges })
			)
		).separated_by(just("\n\n")))
}

fn main() {
	let input = include_str!("../inputs/day5.txt");
	let (seeds, maps) = input_parser().parse(input).unwrap();

	let locations: Vec<u32> = seeds.iter().copied().map(|mut s| {
		for map in &maps {
			s = map.map(s);
		}
		s
	}).collect();
	println!("part 1: {}", locations.into_iter().min().unwrap());

	let seed_ranges: Vec<Ran> = seeds.chunks_exact(2)
		.map(|s| RangeInclusive::new(s[0], s[0]+(s[1]-1))).collect();
	let mut location_ranges: Vec<Ran> = seed_ranges;
	for map in &maps {
		location_ranges = location_ranges.into_iter().flat_map(|ran| map.map_range(ran)).collect();
	}
	println!("part 2: {}", location_ranges.into_iter().map(|ran| *ran.start()).min().unwrap());
}