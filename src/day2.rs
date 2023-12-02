#![feature(trait_alias)]

use chumsky::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Set {
	r: u32,
	g: u32,
	b: u32,
}
impl Set {
	fn max(self, s2: Set) -> Set {
		Set { r: self.r.max(s2.r), g: self.g.max(s2.g), b: self.b.max(s2.b) }
	}
}

#[derive(Debug)]
struct Game {
	id: u32,
	sets: Vec<Set>,
}

trait MyParser<T> = Parser<char, T, Error = Simple<char>>;

fn game() -> impl MyParser<Game> {
	let int = text::int(10).from_str().unwrapped();
	just("Game ").ignore_then(int).then_ignore(just(": ")).then(
		int.then_ignore(just(" ")).then(just("red").or(just("green")).or(just("blue")))
			.separated_by(just(", ")).at_least(1)
			.map(|groups| {
				let mut set = Set { r: 0, g: 0, b: 0 };
				for (cnt, col) in groups {
					match col {
						"red"   => set.r += cnt,
						"green" => set.g += cnt,
						"blue"  => set.b += cnt,
						_ => unreachable!()
					}
				}
				set
			})
			.separated_by(just("; ")).at_least(1)
	).map(|(id, sets)| Game { id, sets })
}

fn main() {
	let input = include_str!("../inputs/day2.txt");
	let mut games = vec![];
	for line in input.lines() {
		games.push(game().parse(line).unwrap());
	}

	let mut id_sum = 0;
	for game in &games {
		if game.sets.iter().all(|s| s.r <= 12 && s.g <= 13 && s.b <= 14) {
			id_sum += game.id;
		}
	}
	println!("part 1: {}", id_sum);

	let mut power_sum = 0;
	for game in games {
		let min_cubes = game.sets.iter().copied().reduce(|s1, s2| s1.max(s2)).unwrap();
		let power = min_cubes.r * min_cubes.g * min_cubes.b;
		power_sum += power;
	}
	println!("part 2: {}", power_sum);
}