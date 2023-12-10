use std::{ops::{IndexMut, Index}, collections::{HashSet, VecDeque}};

struct Grid<T> {
	width: usize,
	data: Box<[T]>,
}
impl<T: Default> Grid<T> {
	fn new_default(width: usize, height: usize) -> Self {
		let mut data = vec![];
		for _ in 0..width*height {
			data.push(Default::default());
		}
		Self { width, data: data.into_boxed_slice() }
	}
}
impl<T> Index<(usize, usize)> for Grid<T> {
	type Output = T;
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.data[index.0 + index.1 * self.width]
	}
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.data[index.0 + index.1 * self.width]
	}
}

const TILE_N: u8 = 0b0001;
const TILE_E: u8 = 0b0010;
const TILE_S: u8 = 0b0100;
const TILE_W: u8 = 0b1000;

fn invert_tile(t: u8) -> u8 {
	((t << 2) & 0xf) | (t >> 2)
}

fn main() {
	let input = include_str!("../inputs/day10.txt");
	let width = input.lines().next().unwrap().len();
	let height = (input.len() + 1) / (width + 1);
	let mut grid = Grid::<u8>::new_default(width, height);
	let mut start = None;
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			grid[(x, y)] = match c {
				'.' => 0,
				'|' => TILE_N | TILE_S,
				'-' => TILE_E | TILE_W,
				'L' => TILE_N | TILE_E,
				'J' => TILE_N | TILE_W,
				'7' => TILE_S | TILE_W,
				'F' => TILE_E | TILE_S,
				'S' => {
					assert!(start.is_none());
					start = Some((x, y));
					0
				},
				_ => unreachable!()
			};
		}
	}
	let start = start.unwrap();
	let (start_x, start_y) = start;
	grid[start] = invert_tile(
		  (grid[(start_x, start_y-1)] & TILE_S)
		| (grid[(start_x+1, start_y)] & TILE_W)
		| (grid[(start_x, start_y+1)] & TILE_N)
		| (grid[(start_x-1, start_y)] & TILE_E)
	);
	
	let mut loop_tiles = HashSet::new();
	let mut queue = VecDeque::new();
	queue.push_back(start);
	loop_tiles.insert(start);
	while let Some((x, y)) = queue.pop_front() {
		let tile = grid[(x,y)];
		let mut neighbors = Vec::with_capacity(4);
		if tile & TILE_N != 0 { neighbors.push((x, y-1)) }
		if tile & TILE_E != 0 { neighbors.push((x+1, y)) }
		if tile & TILE_S != 0 { neighbors.push((x, y+1)) }
		if tile & TILE_W != 0 { neighbors.push((x-1, y)) }
		for neigh in neighbors {
			if !loop_tiles.contains(&neigh) {
				loop_tiles.insert(neigh);
				queue.push_back(neigh);
			}
		}
	}
	let loop_len = loop_tiles.len();
	
	println!("part 1: {}", loop_len/2);
	
	let mut enclosed_cnt = 0;
	for y in 0..height {
		let mut top_inside = false;
		let mut bottom_inside = false;
		for x in 0..width {
			if loop_tiles.contains(&(x, y)) {
				let tile = grid[(x, y)];
				if tile & TILE_N != 0 { top_inside = !top_inside; }
				if tile & TILE_S != 0 { bottom_inside = !bottom_inside; }
			} else if top_inside || bottom_inside {
				assert!(top_inside && bottom_inside);
				enclosed_cnt += 1;
			}
		}
	}
	
	println!("part 2: {}", enclosed_cnt);
}