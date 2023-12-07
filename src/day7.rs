#[derive(Eq, Ord)]
struct Hand {
	cards: [u8; 5],
	bid: u32,
}

impl Hand {
	fn hand_type(&self) -> (u8, u8) {
		let mut counts = [0u8; 13];
		let mut jokers = 0;
		for card in self.cards {
			if card == 1 {
				jokers += 1;
			} else {
				counts[(card-2) as usize] += 1;
			}
		}
		counts.sort_unstable();
		counts[12] += jokers;
		(counts[12], counts[11])
		/* (5,0): five of a kind
		 * (4,1): four of a kind
		 * (3,2): full house
		 * (3,1): three of a kind
		 * (2,2): two pair
		 * (2,1): one pair
		 * (1,1): high card
		 */
	}
}
impl PartialEq for Hand {
	fn eq(&self, other: &Self) -> bool {
		self.cards == other.cards
	}
}
impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some((self.hand_type(), self.cards).cmp(&(other.hand_type(), other.cards)))
	}
}

fn main() {
	let input = include_bytes!("../inputs/day7.txt");
	let mut hands = vec![];
	for line in input.split(|c| *c == b'\n') {
		if line.is_empty() { continue; }
		let mut cards = [0u8; 5];
		for i in 0..5 {
			let c = line[i];
			cards[i] = match c {
				b'2'..=b'9' => c - b'2' + 2,
				b'T' => 10,
				b'J' => 11,
				b'Q' => 12,
				b'K' => 13,
				b'A' => 14,
				_ => unreachable!()
			};
		}
		assert_eq!(line[5], b' ');
		let bid: u32 = std::str::from_utf8(&line[6..]).unwrap().parse().unwrap();
		hands.push(Hand { cards, bid });
	}

	hands.sort();
	let mut score = 0;
	for (i, hand) in hands.iter().enumerate() {
		let rank = i + 1;
		score += (rank as u32) * hand.bid;
	}
	println!("part 1: {}", score);

	for hand in &mut hands {
		for card in &mut hand.cards {
			if *card == 11 { // Jack
				*card = 1; // Joker
			}
		}
	}
	hands.sort();
	let mut score = 0;
	for (i, hand) in hands.iter().enumerate() {
		let rank = i + 1;
		score += (rank as u32) * hand.bid;
	}
	println!("part 2: {}", score);
}