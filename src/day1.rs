fn main() {
	let input = include_str!("../inputs/day1.txt");
	
	let mut calibration_sum = 0;
	for line in input.lines() {
		let mut first_digit = None;
		let mut last_digit = None;
		for c in line.chars() {
			if c.is_digit(10) {
				let digit = c.to_digit(10).unwrap();
				if first_digit.is_none() {
					first_digit = Some(digit);
				}
				last_digit = Some(digit);
			}
		}
		let calibration = first_digit.unwrap() * 10 + last_digit.unwrap();
		calibration_sum += calibration;
	}
	
	println!("part 1: {}", calibration_sum);
	
	let digit_names = [
		("0",0), ("1",1), ("2",2), ("3",3), ("4",4),
		("5",5), ("6",6), ("7",7), ("8",8), ("9",9),
		("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
		("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
	];
	
	let mut calibration_sum = 0;
	for line in input.lines() {
		let mut first_digit = None;
		let mut last_digit = None;
		
		for (pat, digit) in digit_names {
			if let Some(idx) = line.find(pat) {
				if first_digit.map(|(_,idx2)| idx < idx2).unwrap_or(true) {
					first_digit = Some((digit, idx));
				}
			}
			if let Some(idx) = line.rfind(pat) {
				if last_digit.map(|(_,idx2)| idx > idx2).unwrap_or(true) {
					last_digit = Some((digit, idx));
				}
			}
		}
		
		let calibration = first_digit.unwrap().0 * 10 + last_digit.unwrap().0;
		calibration_sum += calibration;
	}
	
	println!("part 2: {}", calibration_sum);
}
