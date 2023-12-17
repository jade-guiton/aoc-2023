use std::collections::{HashMap, VecDeque};

use util::Grid;

#[repr(u8)]
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir { N, E, S, W }

impl Dir {
	fn opposite(&self) -> Self {
		match self {
			Dir::N => Dir::S,
			Dir::E => Dir::W,
			Dir::S => Dir::N,
			Dir::W => Dir::E,
		}
	}
	fn apply(&self, x: i32, y: i32, w: i32, h: i32, d: i32) -> Option<(i32, i32)> {
		let (dx, dy) = match self {
			Dir::N => (0, -1),
			Dir::E => (1, 0),
			Dir::S => (0, 1),
			Dir::W => (-1, 0),
		};
		let (x2, y2) = (x + dx*d, y + dy*d);
		if 0 <= x2 && x2 < w && 0 <= y2 && y2 < h { Some((x2, y2)) } else { None }
	}
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
	x: i32,
	y: i32,
	prev_dir: Dir,
	counter: u8,
}

fn compute_min_heat_loss(w: i32, h: i32, grid: &Grid::<u16>, can_move: impl Fn(&State, Dir) -> bool, move_dist: impl Fn(&State, Dir) -> u8) -> u16 {
	let mut min_loss = HashMap::<State, u16>::new();
	let mut frontier = VecDeque::new();
	let init_state = State {
		x: 0, y: 0,
		prev_dir: Dir::N, // arbitrary
		counter: 0
	};
	min_loss.insert(init_state.clone(), 0);
	frontier.push_back(init_state);
	let mut res = None;
	while let Some(state) = frontier.pop_front() {
		let loss = min_loss[&state];
		if state.x == w-1 && state.y == h-1 {
			res = Some(loss);
			break;
		}
		let mut sort_needed = false;
		for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
			if state.counter != 0 {
				if dir == state.prev_dir.opposite() { continue; }
				if !can_move(&state, dir) { continue; }
			}
			let dist = move_dist(&state, dir);
			if let Some((x2, y2)) = dir.apply(state.x, state.y, w as i32, h as i32, dist as i32) {
				// println!("{};{} -> {};{}", state.x, state.y, x2, y2);
				let state2 = State {
					x: x2, y: y2,
					prev_dir: dir,
					counter: if dir == state.prev_dir { state.counter + dist } else { dist }
				};
				let loss2 = loss + (1..=dist).map(|i|
					grid[dir.apply(state.x, state.y, w as i32, h as i32, i as i32).unwrap()]
				).sum::<u16>();
				if min_loss.get(&state2).map(|loss3| loss2 < *loss3).unwrap_or(true) {
					if !frontier.contains(&state2) {
						frontier.push_back(state2.clone());
					}
					min_loss.insert(state2, loss2);
					sort_needed = true;
				}
			}
		}
		if sort_needed {
			frontier.make_contiguous().sort_by_key(|s| min_loss[s]);
		}
	}
	res.unwrap()
}

fn main() {
	let input = include_bytes!("../inputs/day17.txt");
	let grid = Grid::load_from_bytes(input, |c,_,_| (c - b'0') as u16);
	let (w, h) = (grid.width, grid.height);
	
	let res = compute_min_heat_loss(w, h, &grid,
		|state, dir| state.counter < 3 || dir != state.prev_dir,
		|_state, _dir| 1
	);
	println!("part 1: {:?}", res);
	
	let res = compute_min_heat_loss(w, h, &grid,
		|state, dir| (dir == state.prev_dir && state.counter < 10)
			|| (dir != state.prev_dir && state.counter >= 4),
		|state, dir| if dir != state.prev_dir || state.counter == 0 { 4 } else { 1 });
	println!("part 2: {:?}", res);
}