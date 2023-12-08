use std::collections::{HashMap, HashSet};

use chumsky::prelude::*;

fn node_id() -> impl Parser<char, u16, Error=Simple<char>> {
	one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ").repeated().exactly(3)
		.map(|chars| chars.into_iter().fold(0, |id, c| id*32 + (c as u32 - 'A' as u32) as u16))
}

fn main() {
	let input = include_str!("../inputs/day8.txt");
	let (path, network): (Vec<u8>, HashMap<u16, [u16; 2]>) =
		one_of("LR").map(|c| if c == 'R' { 1u8 } else { 0u8 }).repeated()
		.then_ignore(just("\n\n"))
		.then(
			node_id()
				.then_ignore(just(" = (")).then(node_id())
				.then_ignore(just(", ")).then(node_id())
				.then_ignore(just(")"))
				.separated_by(just("\n")).allow_trailing()
				.map(|nodes| {
					let mut map = HashMap::<u16, [u16;2]>::new();
					for ((id, left), right) in nodes {
						map.insert(id, [left, right]);
					}
					map
				})
		).parse(input).unwrap();
	
	let start = 0; // AAA
	let end = 25 + 32*(25 + 32*25); // ZZZ
	let path_len = path.len();

	let mut cur = start;
	let mut steps = 0;
	let mut i = 0;
	while cur != end {
		cur = network.get(&cur).unwrap()[path[i] as usize];
		i = (i + 1) % path_len;
		steps += 1;
	}

	println!("part 1: {}", steps);

	let mut start_nodes = HashSet::new();
	for node in network.keys() {
		if node & 0x1f == 0 { // ends in A
			start_nodes.insert(*node);
		}
	}

	let mut loop_sizes = vec![];
	for start in start_nodes {
		let mut final_states = vec![];
		let mut visited = HashMap::new();
		let mut cur = start;
		let mut steps = 0;
		while !visited.contains_key(&cur) {
			visited.insert(cur, steps);
			for i in 0..path_len {
				cur = network.get(&cur).unwrap()[path[i] as usize];
				steps += 1;
				if cur & 0x1f == 25 { // ends in Z -> final state
					final_states.push(steps);
				}
			}
		}
		let loop_point = *visited.get(&cur).unwrap();
		let loop_size = steps - loop_point;
		// We now know all steps at which a final state is visited by this ghost:
		// final_states[< loop_point], and then visited[>= loop_point] + k * loop_size

		// We notice a pattern that will simplify the rest of the computation:
		assert!(final_states.len() == 1);
		assert!(final_states[0] == loop_size);
		// This means this ghost will be in a final state precisely at steps that
		// are a non-zero multiple of loop_size, which is itself a multiple of path_len.
		loop_sizes.push(loop_size / path_len);
	}

	// In principle, we just need to find the LCM of the loop sizes.
	// But since it seems they are all coprime after dividing by path_len...
	println!("part 2: {}", loop_sizes.iter().product::<usize>() * path_len);
}