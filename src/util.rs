pub struct Grid<T> {
	pub width: i32,
	pub height: i32,
	pub data: Box<[T]>,
}
impl<T> Grid<T> {
	pub fn load_from_bytes(input: &[u8], mut f: impl FnMut(u8, i32, i32) -> T) -> Self {
		let width = input.iter().position(|c| *c == b'\n').unwrap() as i32;
		let height = (input.len() as i32 + 1) / (width + 1);
		let mut data = Vec::with_capacity((width*height) as usize);
		for (y, line) in (0..height).zip(input.split(|c| *c == b'\n')) {
			for (x, c) in line.iter().enumerate() {
				data.push(f(*c, x as i32, y as i32));
			}
		}
		Grid { width, height, data: data.into_boxed_slice() }
	}
}
impl<T: Clone> Grid<T> {
	pub fn new(width: i32, height: i32, val: T) -> Self {
		Self { width, height, data: vec![val; (width*height) as usize].into_boxed_slice() }
	}
}
impl<T> std::ops::Index<(i32, i32)> for Grid<T> {
	type Output = T;
	fn index(&self, index: (i32, i32)) -> &Self::Output {
		&self.data[(index.0 + index.1 * self.width) as usize]
	}
}
impl<T> std::ops::IndexMut<(i32, i32)> for Grid<T> {
	fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
		&mut self.data[(index.0 + index.1 * self.width) as usize]
	}
}
