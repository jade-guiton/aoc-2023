#![feature(trait_alias)]
use chumsky::prelude::*;
trait MyParser<T> = Parser<char, T, Error=Simple<char>>;
fn int() -> impl MyParser<i64> {
	just(" ").repeated().ignore_then(text::int(10).from_str().unwrapped())
}

fn calculate_margin(time: i64, record: i64) -> i64 {
	// if we hold the button for N ms, we will travel D(N) = max(0,time-N) * N mm
	// D(N) > record  <=>  time*N - N^2 > record  <=>  -N^2 + time*N - record > 0
	let delta = time*time - 4*record;
	if delta < 0 {
		return 0;
	}
	let sqrt_delta = (delta as f64).sqrt();
	// <=>  N between (time +/- sqrt(Delta)) / 2
	let min_hold = ((time as f64 - sqrt_delta) / 2.0).ceil() as i64;
	let max_hold = ((time as f64 + sqrt_delta) / 2.0).floor() as i64;
	max_hold - min_hold + 1
}

fn concat_ints(a: i64, b: i64) -> i64 {
	assert!(a >= 0 && b >= 0);
	a * 10i64.pow(b.ilog10() + 1) + b
}

fn main() {
	let input = include_str!("../inputs/day6.txt");
	let (times, records) = just("Time:").ignore_then(int().repeated())
		.then_ignore(just("\nDistance:")).then(int().repeated())
		.parse(input).unwrap();
	let races: Vec<_> = times.into_iter().zip(records.into_iter()).collect();

	let mut part1 = 1;
	for (time, record) in races.iter().copied() {
		part1 *= calculate_margin(time, record);
	}
	println!("part 1: {}", part1);

	let (time, record) = races.into_iter().reduce(|(t1,r1),(t2,r2)|
		(concat_ints(t1,t2), concat_ints(r1,r2))).unwrap();
	println!("part 2: {}", calculate_margin(time, record));
}