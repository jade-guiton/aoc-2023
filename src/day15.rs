#![feature(slice_split_once)]

fn hash(s: &[u8]) -> u8 {
	let mut cur = 0u8;
	for c in s {
		cur = cur.wrapping_add(*c).wrapping_mul(17);
	}
	cur
}

#[derive(Clone, Debug)]
struct Pair<'a> {
	label: &'a [u8],
	focal: i8,
}

fn main() {
	let input = include_bytes!("../inputs/day15.txt");
	
	println!("part 1: {}", input.split(|c| *c == b',').map(|step| hash(step) as u32).sum::<u32>());
	
	let mut boxes: [Vec<Pair>; 256] = vec![vec![]; 256].try_into().unwrap();
	for step in input.split(|c| *c == b',') {
		let (label, focal) = if step[step.len()-1] == b'-' {
			(&step[0..step.len()-1], -1)
		} else {
			assert!(step[step.len()-2] == b'=');
			(&step[0..step.len()-2], (step[step.len()-1] - b'0') as i8)
		};
		let box_ref = &mut boxes[hash(label) as usize];
		let label_pos = box_ref.iter().position(|p| p.label == label);
		if focal != -1 {
			if let Some(i) = label_pos {
				box_ref[i].focal = focal;
			} else {
				box_ref.push(Pair { label, focal });
			}
		} else if let Some(i) = label_pos {
			box_ref.remove(i);
		}
	}
	let power = boxes.into_iter().enumerate().map(|(box_idx, pairs)|
		(1 + box_idx) * pairs.into_iter().enumerate().map(|(slot_idx, pair)|
			(1 + slot_idx) * (pair.focal as usize)
		).sum::<usize>()
	).sum::<usize>();
	println!("part 2: {}", power);
}