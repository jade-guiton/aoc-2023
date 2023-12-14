use std::collections::HashMap;

const SIZE: usize = 100;

struct Grid {
	cubic_rocks: [u128; SIZE],
	round_rocks: [u128; SIZE],
}

fn roll_north(grid: &mut Grid) {
	let mut wall_dist = [0u8; SIZE];
	for row in 0..SIZE {
		let row_cubic = grid.cubic_rocks[row];
		let mut row_round = grid.round_rocks[row];
		for col in 0..SIZE {
			if (row_cubic >> col) & 1 != 0 {
				wall_dist[col] = 0;
			} else if (row_round >> col) & 1 != 0 {
				if wall_dist[col] > 0 {
					row_round &= !(1u128 << col);
					grid.round_rocks[row - wall_dist[col] as usize] |= 1u128 << col;
				}
			} else {
				wall_dist[col] += 1;
			}
		}
		grid.round_rocks[row] = row_round;
	}
}

fn calculate_load(grid: &Grid) -> u32 {
	let mut load = 0;
	for row in 0..SIZE {
		load += grid.round_rocks[row].count_ones() * (SIZE - row) as u32;
	}
	load
}

fn rotate_masks(grid: &[u128; SIZE]) -> [u128; SIZE] {
	let mut new_grid = [0u128; SIZE];
	for row in 0..SIZE {
		let mut row_mask = grid[row];
		for col in 0..SIZE {
			new_grid[col] |= (row_mask & 1) << (SIZE-1-row);
			row_mask >>= 1;
		}
	}
	new_grid
}
fn spin_cycle(grid: &mut Grid) {
	for _ in 0..4 {
		roll_north(grid);
		grid.cubic_rocks = rotate_masks(&grid.cubic_rocks);
		grid.round_rocks = rotate_masks(&grid.round_rocks);
	}
}

fn main() {
	let input = include_bytes!("../inputs/day14.txt");
	let mut grid = Grid {
		cubic_rocks: [0; SIZE],
		round_rocks: [0; SIZE],
	};
	for (row, line) in input.split(|b| *b == b'\n').enumerate() {
		assert!(line.len() == SIZE);
		for (col, cell) in line.iter().enumerate() {
			if *cell == b'#' {
				grid.cubic_rocks[row] |= 1u128 << col;
			}
			if *cell == b'O' {
				grid.round_rocks[row] |= 1u128 << col;
			}
		}
	}
	
	roll_north(&mut grid);
	println!("part 1: {}", calculate_load(&grid));
	
	let mut history = HashMap::<[u128; SIZE], usize>::new();
	let mut max_step = 1_000_000_000;
	let mut step = 0;
	while step < max_step {
		if let Some(prev_step) = history.insert(grid.round_rocks, step) {
			// cycle detected!
			max_step = step + (1_000_000_000 - prev_step) % (step - prev_step);
		}
		spin_cycle(&mut grid);
		step += 1;
	}
	println!("part 2: {}", calculate_load(&grid));
}