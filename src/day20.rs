use std::{collections::{HashMap, VecDeque}, cell::RefCell};

use chumsky::prelude::*;

#[derive(PartialEq, Eq, Debug)]
enum NodeType {
	Broadcaster,
	Conjunction(u8, u8),
	Flipflop,
	Output,
}
#[derive(Debug)]
struct Node {
	ty: NodeType,
	st: bool,
	out: Vec<u16>,
}

fn parse_letter(c: char) -> u16 {
	c as u16 - 'a' as u16
}
fn parse_node_name() -> impl Parser<char, u16, Error=Simple<char>> {
	filter(|c| ('a'..='z').contains(c)).repeated().exactly(2)
		.map(|n| {
			if n == ['r','x'] {
				1
			} else {
				parse_letter(n[0]) * 26 + parse_letter(n[1]) + 2
			}
		})
}

fn parse_node() -> impl Parser<char, (u16, Node), Error=Simple<char>> {
	choice((
		just("broadcaster").map(|_| (NodeType::Broadcaster, 0)),
		one_of("%&").map(|c| match c {
			'%' => NodeType::Flipflop,
			'&' => NodeType::Conjunction(0, 0),
			_ => unreachable!()
		}).then(parse_node_name())
	)).then_ignore(just(" -> ")).then(
		parse_node_name().separated_by(just(", "))
	).map(|((ty, id), out)| (id, Node { ty, st: false, out }))
}

enum SimResult {
	OutputTriggered,
	Finished { lo: usize, hi: usize },
}

fn press_button(nodes: &mut [Node]) -> SimResult {
	let mut lo_pulses = 0;
	let mut hi_pulses = 0;
	let mut pulses = VecDeque::new();
	pulses.push_back((0, false, false)); // broadcast
	lo_pulses += 1;
	while let Some((id, prev_val, val)) = pulses.pop_front() {
		let node = &mut nodes[id as usize];
		let mut new_st = None;
		match &mut node.ty {
			NodeType::Broadcaster => new_st = Some(false),
			NodeType::Output => {
				if val == false {
					return SimResult::OutputTriggered;
				}
			},
			NodeType::Conjunction(ctr, max) => {
				if val > prev_val {
					*ctr += 1;
				} else if val < prev_val {
					*ctr -= 1;
				}
				new_st = Some(*ctr < *max);
			},
			NodeType::Flipflop => {
				if !val {
					new_st = Some(!node.st);
				}
			}
		}
		if let Some(new_st) = new_st {
			for id2 in &node.out {
				//println!("{} -> {} ({})", node_name(id), node_name(*id2), if new_st { "hi" } else { "lo" });
				pulses.push_back((*id2, node.st, new_st));
			}
			if new_st {
				hi_pulses += node.out.len();
			} else {
				lo_pulses += node.out.len();
			}
			node.st = new_st;
		}
	}
	SimResult::Finished { lo: lo_pulses, hi: hi_pulses }
}

fn main() {
	let input = include_str!("../inputs/day20.txt");
	let mut nodes = HashMap::new();
	for line in input.lines() {
		let (id, node) = parse_node().parse(line).unwrap();
		nodes.insert(id, RefCell::new(node));
	}
	nodes.insert(1, RefCell::new(Node { ty: NodeType::Output, st: false, out: vec![] }));
	
	for node in nodes.values() {
		let node = node.borrow();
		for id2 in &node.out {
			let mut node2 = nodes.get(&id2).map(|n| n.borrow_mut()).unwrap();
			if let NodeType::Conjunction(_ctr, max) = &mut node2.ty {
				*max += 1;
			}
		}
	}
	
	let mut node_ids: Vec<u16> = nodes.keys().copied().collect();
	node_ids.sort();
	let mut nodes = {
		let mut nodes_vec = vec![];
		for id in &node_ids {
			let mut node = nodes.remove(&id).unwrap().into_inner();
			for id2 in &mut node.out {
				*id2 = node_ids.iter().position(|id3| id3 == id2).unwrap() as u16;
			}
			nodes_vec.push(node);
		}
		nodes_vec
	};
	
	let mut lo_pulses = 0;
	let mut hi_pulses = 0;
	for _ in 0..1000 {
		if let SimResult::Finished { lo, hi } = press_button(&mut nodes) {
			lo_pulses += lo;
			hi_pulses += hi;
		} else {
			unreachable!();
		}
	}
	
	println!("part 1: {}", lo_pulses * hi_pulses);
	
	// Reset
	for node in &mut nodes {
		node.st = false;
		if let NodeType::Conjunction(ctr, _max) = &mut node.ty {
			*ctr = 0;
		}
	}
	
	let mut presses = 0;
	loop {
		presses += 1;
		if let SimResult::OutputTriggered = press_button(&mut nodes) {
			break
		}
		if presses % 10_000_000 == 0 {
			println!("{}M presses", presses/1_000_000);
		}
	}
	
	println!("part 2: {}", presses);
}