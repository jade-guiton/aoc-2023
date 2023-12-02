use pom::utf8::{Parser, one_of, sym, seq, list};

#[derive(Debug, Clone, Copy)]
struct Set {
	r: u32,
	g: u32,
	b: u32,
}
impl Set {
	fn add(self, s2: Set) -> Set {
		Set { r: self.r + s2.r, g: self.g + s2.g, b: self.b + s2.b }
	}
	fn max(self, s2: Set) -> Set {
		Set { r: self.r.max(s2.r), g: self.g.max(s2.g), b: self.b.max(s2.b) }
	}
}
#[derive(Debug)]
struct Game {
	id: u32,
	sets: Vec<Set>,
}

fn int<'a>() -> Parser<'a, u32> {
	one_of("0123456789").repeat(1..).collect()
		.convert(str::parse::<u32>)
}
fn game<'a>() -> Parser<'a, Game> {
	let group = (int() - sym(' ') + (seq("red") | seq("green") | seq("blue")))
		.map(|(cnt, col)| match col {
			"red" => Set { r: cnt, g: 0, b: 0 },
			"green" => Set { r: 0, g: cnt, b: 0 },
			"blue" => Set { r: 0, g: 0, b: cnt },
			_ => unreachable!()
		});
	let set = list(group, seq(", "))
		.map(|groups| groups.iter().copied().reduce(|g1,g2| g1.add(g2)).unwrap());
	(seq("Game ") * int() - seq(": ") + list(set, seq("; ")))
		.map(|(id, sets)| Game { id, sets })
}

fn main() {
	let input = include_str!("../inputs/day2.txt");
	let mut games = vec![];
	for line in input.lines() {
		games.push(game().parse_str(line).unwrap());
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