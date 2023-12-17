use std::collections::VecDeque;

use util::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis { Hor, Ver }

#[derive(Clone, PartialEq, Eq)]
struct State { x: i32, y: i32, from_axis: Axis }

fn get_min_loss<'a>(min_loss: &'a mut Grid<(u16, u16)>, state: &State) -> &'a mut u16 {
	let pair = &mut min_loss[(state.x, state.y)];
	match state.from_axis { Axis::Hor => &mut pair.0, Axis::Ver => &mut pair.1 }
}
fn compute_min_heat_loss(w: i32, h: i32, grid: &Grid::<u16>, min_dist: i32, max_dist: i32) -> u16 {
	let mut min_loss = Grid::<(u16, u16)>::new(w, h, (std::u16::MAX, std::u16::MAX));
	let mut frontier = VecDeque::new();
	min_loss[(0, 0)] = (0, 0);
	frontier.push_back(State { x: 0, y: 0, from_axis: Axis::Hor });
	frontier.push_back(State { x: 0, y: 0, from_axis: Axis::Ver });
	let mut res = None;
	while let Some(state) = frontier.pop_front() {
		let loss = *get_min_loss(&mut min_loss, &state);
		if state.x == w-1 && state.y == h-1 {
			res = Some(loss);
			break;
		}
		let new_axis = match state.from_axis { Axis::Hor => Axis::Ver, Axis::Ver => Axis::Hor };
		let mut sort_needed = false;
		for dir in [1i32, -1i32] {
			let mut state2 = State {
				x: state.x, y: state.y,
				from_axis: new_axis,
			};
			let mut loss2 = loss;
			for dist in 1..=max_dist {
				match new_axis {
					Axis::Hor => state2.x += dir,
					Axis::Ver => state2.y += dir,
				};
				if state2.x < 0 || state2.y < 0 || state2.x >= w || state2.y >= h { break; }
				loss2 += grid[(state2.x, state2.y)];
				if dist < min_dist { continue; }
				let loss3 = get_min_loss(&mut min_loss, &state2);
				if loss2 < *loss3 {
					if !frontier.contains(&state2) {
						frontier.push_back(state2.clone());
					}
					*loss3 = loss2;
					sort_needed = true;
				}
			}
		}
		if sort_needed {
			frontier.make_contiguous().sort_by_key(|s| *get_min_loss(&mut min_loss, s));
		}
	}
	res.unwrap()
}

fn main() {
	let input = include_bytes!("../inputs/day17.txt");
	let grid = Grid::load_from_bytes(input, |c,_,_| (c - b'0') as u16);
	let (w, h) = (grid.width, grid.height);
	
	let res = compute_min_heat_loss(w, h, &grid, 1, 3);
	println!("part 1: {:?}", res);
	
	let res = compute_min_heat_loss(w, h, &grid, 4, 10);
	println!("part 2: {:?}", res);
}