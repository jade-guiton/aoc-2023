struct Grid<T> {
	width: usize,
	data: Box<[T]>,
}
impl<T: Clone> Grid<T> {
	fn new(w: usize, h: usize, val: T) -> Self {
		Self { width: w, data: vec![val; w*h].into_boxed_slice() }
	}
}
impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
	type Output = T;
	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.data[index.0 + index.1 * self.width]
	}
}
impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.data[index.0 + index.1 * self.width]
	}
}

#[derive(Clone)]
enum Tile {
	Empty,
	MirrorSlash,
	MirrorBackslash,
	SplitterHor,
	SplitterVer,
}

struct Beam {
	x: i8, y: i8,
	dx: i8, dy: i8,
}

#[derive(Clone)]
struct BeamSet {
	mask: u8,
}
impl BeamSet {
	const EMPTY: BeamSet = BeamSet { mask: 0 };
	fn dir_idx(dx: i8, dy: i8) -> i8 {
		(dx + (dy+1)*3) / 2
	}
	fn has_dir(&self, dx: i8, dy: i8) -> bool {
		self.mask & (1 << BeamSet::dir_idx(dx,dy)) != 0
	}
	fn add_dir(&mut self, dx: i8, dy: i8) {
		self.mask |= 1 << BeamSet::dir_idx(dx,dy);
	}
	fn is_energized(&self) -> bool {
		self.mask != 0
	}
}

fn count_energized(w: usize, h: usize, tiles: &Grid::<Tile>, beam: Beam) -> u32 {
	let mut beams = Grid::<BeamSet>::new(w, h, BeamSet::EMPTY);
	let mut beam_fronts = vec![beam];
	while !beam_fronts.is_empty() {
		let mut i = 0;
		while i < beam_fronts.len() {
			let beam = &mut beam_fronts[i];
			if beam.x < 0 || beam.y < 0 || beam.x >= w as i8 || beam.y >= h as i8 {
				beam_fronts.remove(i);
				continue;
			}
			let pos = (beam.x as usize, beam.y as usize);
			if beams[pos].has_dir(beam.dx, beam.dy) {
				beam_fronts.remove(i);
				continue;
			}
			beams[pos].add_dir(beam.dx, beam.dy);
			let mut new_beam = None;
			match tiles[pos] {
				Tile::MirrorSlash => (beam.dx, beam.dy) = (-beam.dy, -beam.dx),
				Tile::MirrorBackslash => (beam.dx, beam.dy) = (beam.dy, beam.dx),
				Tile::SplitterHor if beam.dy != 0 => {
					new_beam = Some(Beam { x: beam.x + 1, y: beam.y, dx: 1, dy: 0 });
					(beam.dx, beam.dy) = (-1, 0);
				},
				Tile::SplitterVer if beam.dx != 0 => {
					new_beam = Some(Beam { x: beam.x, y: beam.y + 1, dx: 0, dy: 1 });
					(beam.dx, beam.dy) = (0, -1);
				},
				_ => {},
			}
			beam.x += beam.dx;
			beam.y += beam.dy;
			if let Some(new_beam) = new_beam {
				beam_fronts.push(new_beam);
			}
			i += 1;
		}
	}
	
	let mut energized = 0;
	for y in 0..h {
		for x in 0..w {
			if beams[(x, y)].is_energized() {
				energized += 1;
			}
		}
	}
	energized
}

fn main() {
	let input = include_bytes!("../inputs/day16.txt");
	let w = input.iter().position(|c| *c == b'\n').unwrap();
	let h = (input.len() + 1) / (w + 1);
	let mut tiles = Grid::<Tile>::new(w, h, Tile::Empty);
	for (y, line) in input.split(|c| *c == b'\n').enumerate() {
		if line.len() == 0 { continue; }
		for (x, c) in line.iter().enumerate() {
			tiles[(x, y)] = match c {
				b'.' => Tile::Empty,
				b'/' => Tile::MirrorSlash,
				b'\\' => Tile::MirrorBackslash,
				b'-' => Tile::SplitterHor,
				b'|' => Tile::SplitterVer,
				_ => unreachable!(),
			};
		}
	}
	
	println!("part 1: {}", count_energized(w, h, &tiles, Beam { x: 0, y: 0, dx: 1, dy: 0 }));
	
	let (mx, my) = ((w-1) as i8, (h-1) as i8);
	let beams =
		       (0..=mx).map(|x| Beam { x,    y:0,  dx:0,  dy:1  })
		.chain((0..=mx).map(|x| Beam { x,    y:my, dx:0,  dy:-1 }))
		.chain((0..=my).map(|y| Beam { x:0,  y,    dx:1,  dy:0  }))
		.chain((0..=my).map(|y| Beam { x:mx, y,    dx:-1, dy:0  }));
	println!("part 2: {}", beams.map(|beam| count_energized(w, h, &tiles, beam)).max().unwrap());
}
