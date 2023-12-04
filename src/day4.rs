#![feature(trait_alias)]

use std::collections::VecDeque;

use chumsky::{primitive::just, Parser, text, error::Simple};

struct Card {
	winning: Vec<u8>,
	values: Vec<u8>,
}

trait MyParser<T> = Parser<char, T, Error = Simple<char>>;
fn int() -> impl MyParser<u8> {
	just(" ").repeated().ignore_then(text::int(10).from_str().unwrapped())
}
fn card() -> impl MyParser<Card> {
	just("Card").ignore_then(int())
		.then_ignore(just(":"))
		.then(int().repeated())
		.then_ignore(just(" |"))
		.then(int().repeated())
		.map(|((_no, winning), values)| {
			Card { winning, values }
		})
}

fn main() {
	let input = include_str!("../inputs/day4.txt");
	let mut total_points = 0;
	let mut total_cards = 0;
	let mut extras = VecDeque::new();
	for line in input.lines() {
		let card = card().parse(line).unwrap();
		let mut winners = 0;
		for number in card.values {
			if card.winning.contains(&number) {
				winners += 1;
			}
		}

		total_points += if winners == 0 { 0 } else { 1 << (winners-1) };

		let card_cnt = extras.pop_front().unwrap_or(0) + 1;
		total_cards += card_cnt;
		for i in 0..winners {
			if i >= extras.len() {
				extras.push_back(card_cnt);
			} else {
				extras[i] += card_cnt;
			}
		}
	}

	println!("part 1: {}", total_points);
	println!("part 2: {}", total_cards);
}