fn compute_offsets(empty: &[bool]) -> Vec<usize> {
	let mut offsets = Vec::with_capacity(empty.len());
	let mut cur_offset = 0;
	for is_empty in empty.iter().copied() {
		offsets.push(cur_offset);
		if is_empty {
			cur_offset += 1;
		}
	}
	offsets
}

fn compute_distance_sum(
		pts: &[(usize, usize)],
		x_off: &[usize], y_off: &[usize],
		off_mult: usize
) -> usize {
	let mut dist_sum = 0;
	for (i, (x1, y1)) in pts.iter().enumerate() {
		let (x1, y1) = (x1 + x_off[*x1]*off_mult, y1 + y_off[*y1]*off_mult);
		for (x2, y2) in &pts[0..i] {
			let (x2, y2) = (x2 + x_off[*x2]*off_mult, y2 + y_off[*y2]*off_mult);
			dist_sum += x2.abs_diff(x1) + y2.abs_diff(y1);
		}
	}
	dist_sum
}

fn main() {
	let input = include_str!("../inputs/day11.txt");
	
	let width = input.lines().next().unwrap().len();
	let height = (input.len() + 1) / (width + 1);
	let mut empty_cols = vec![true; width];
	let mut empty_rows = vec![true; height];
	
	let mut galaxies = vec![];
	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c == '#' {
				galaxies.push((x, y));
				empty_cols[x] = false;
				empty_rows[y] = false;
			}
		}
	}
	
	let col_offset = compute_offsets(&empty_cols);
	let row_offset = compute_offsets(&empty_rows);
	
	println!("part 1: {}", compute_distance_sum(&galaxies, &col_offset, &row_offset, 1));
	println!("part 2: {}", compute_distance_sum(&galaxies, &col_offset, &row_offset, 999_999));
}