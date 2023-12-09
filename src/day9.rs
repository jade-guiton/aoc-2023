fn main() {
	let input = include_str!("../inputs/day9.txt");
	let mut extrap_sum = 0;
	let mut extrap2_sum = 0;
	for line in input.lines() {
		let mut values = vec![];
		let mut rest = line;
		while !rest.is_empty() {
			let space = rest.find(' ');
			let next = space.map(|i| i+1).unwrap_or(rest.len());
			let end = space.unwrap_or(rest.len());
			let value: i32 = rest[..end].parse().unwrap();
			values.push(value);
			rest = &rest[next..];
		}
		
		let mut derivatives = vec![values.into_boxed_slice()];
		loop {
			let seq = derivatives.last().unwrap();
			let mut der = Vec::with_capacity(seq.len()-1);
			for i in 0..seq.len()-1 {
				der.push(seq[i+1] - seq[i]);
			}
			let der = der.into_boxed_slice();
			if der.iter().all(|x| *x == 0) { break; }
			derivatives.push(der);
		}

		let extrap: i32 = derivatives.iter().map(|seq| seq.last().unwrap()).sum();
		extrap_sum += extrap;

		extrap2_sum += derivatives.iter().rev().fold(0,
			|acc, seq| seq.first().unwrap() - acc);
	}
	println!("part 1: {}", extrap_sum);
	println!("part 2: {}", extrap2_sum);
}