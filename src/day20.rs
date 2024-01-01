use std::collections::HashMap;

use chumsky::prelude::*;

#[derive(PartialEq, Eq, Debug)]
enum NodeType {
	Input,
	Nand,
	Flipflop,
	Output,
}
#[derive(Debug)]
struct Node {
	id: String,
	ty: NodeType,
	out: Vec<String>,
}

fn parse_node_name() -> impl Parser<char, String, Error=Simple<char>> {
	filter(|c| ('a'..='z').contains(c)).repeated().at_least(1).collect()
}
fn parse_node() -> impl Parser<char, Node, Error=Simple<char>> {
	choice((
		just("broadcaster").map(|_| (NodeType::Input, "broadcaster".to_owned())),
		one_of("%&").map(|c| match c {
			'%' => NodeType::Flipflop,
			'&' => NodeType::Nand,
			_ => unreachable!()
		}).then(parse_node_name())
	)).then_ignore(just(" -> ")).then(
		parse_node_name().separated_by(just(", "))
	).map(|((ty, id), out)| Node { id, ty, out })
}

fn main() {
	let input = include_str!("../inputs/day20.txt");
	let mut nodes = HashMap::new();
	for line in input.lines() {
		let node = parse_node().parse(line).unwrap();
		nodes.insert(node.id.clone(), node);
	}
	nodes.insert("rx".to_owned(), Node { id: "rx".to_owned(), ty: NodeType::Output, out: vec![] });
	
	// Check that input has the expected structure:
	
	// The output is driven by a single NAND gate
	let final_nand_id = {
		let final_nodes: Vec<&Node> = nodes.values()
			.filter(|n| n.out.contains(&"rx".to_owned())).collect();
		assert!(final_nodes.len() == 1);
		let final_node = final_nodes[0];
		assert!(final_node.ty == NodeType::Nand);
		assert!(final_node.out.len() == 1);
		final_node.id.clone()
	};
	
	// Broadcaster is connected to multiple binary counters (flip-flop chains)
	let mut ctr_moduli = vec![];
	for mut ff_id in &nodes.get("broadcaster").unwrap().out {
		let mut ff = nodes.get(ff_id).unwrap();
		// The first flip flop always outputs to a NAND
		let nand_id = {
			let nands: Vec<String> = ff.out.iter()
				.filter(|id| nodes.get(*id).unwrap().ty == NodeType::Nand)
				.cloned()
				.collect();
			assert!(nands.len() == 1);
			nands.into_iter().next().unwrap()
		};
		let nand = nodes.get(&nand_id).unwrap();
		// The chain is made of 12 flip-flops forming a binary counter
		let mut ctr_modulus: u16 = 0;
		let mut nand_outputs = vec![];
		for i in 0..12 {
			assert!(ff.ty == NodeType::Flipflop);
			// The flip-flops may output to the NAND, or the NAND may output to it
			let to_nand = ff.out.iter().any(|id| *id == nand_id);
			let from_nand = nand.out.iter().any(|id| id == ff_id);
			if i == 0 {
				assert!(from_nand && to_nand); // The first flip-flop has both links
			} else {
				assert!(from_nand ^ to_nand); // The others have exactly one
			}
			if to_nand {
				ctr_modulus |= 1 << i; // Store a 1 bit
			}
			if from_nand {
				nand_outputs.push(ff_id);
			}
			if i == 11 { // Last flip-flop
				assert!(ff.out.len() == if to_nand { 1 } else { 0 }); // Nothing after it
			} else {
				assert!(ff.out.len() == if to_nand { 2 } else { 1 }); // Another flip-flop after it
				ff_id = ff.out.iter().find(|id| **id != nand_id).unwrap();
				ff = nodes.get(ff_id).unwrap();
			}
		}
		// Besides the ones already checked, the NAND has one more output
		assert!(nand.out.len() == 1 + nand_outputs.len());
		let nand2_id = nand.out.iter().find(|id| !nand_outputs.contains(id)).unwrap();
		// That output is another NAND, acting as an inverter, connected to the final NAND
		let nand2 = nodes.get(nand2_id).unwrap();
		assert!(nand2.ty == NodeType::Nand);
		assert!(nand2.out.len() == 1);
		assert!(nand2.out[0] == final_nand_id);
		
		// The bit pattern we extracted is the period/modulus of this binary counter
		ctr_moduli.push(ctr_modulus);
	}
	
	// Based on this structure, we can calculate the number of pulses generated
	// for each button press.
	
	// No chance at a counter reset within 1000 button presses
	assert!(ctr_moduli.iter().all(|ctr_mod| *ctr_mod > 1000));
	
	let ctr_nb = ctr_moduli.len();
	let mut ctr_vals: Vec<u16> = vec![0; ctr_nb];
	let mut pulses = [0, 0];
	for _ in 0..1000 {
		pulses[0] += 1; // From button to broadcaster
		
		for ctr_i in 0..ctr_nb {
			let ctr_mod = ctr_moduli[ctr_i];
			let val = &mut ctr_vals[ctr_i];
			
			pulses[0] += 1; // From broadcaster
			
			let carries = val.trailing_ones();
			let mut nand_updates = 0;
			for ff_i in 0..=carries {
				let out = if ff_i < carries { 0 } else { 1 };
				if (ctr_mod >> ff_i) & 1 == 1 { // If outputs to NAND
					pulses[out] += 1; // From FF to NAND
					nand_updates += 1;
				}
				if ff_i < 11 { // Not last in chain
					pulses[out] += 1;
				}
			}
			
			let nand_outputs = 14-ctr_mod.count_ones();
			pulses[1] += nand_updates * nand_outputs; // From NAND to FFs (ignored) + inverter
			pulses[0] += nand_updates; // From inverter to final NAND
			pulses[1] += nand_updates; // From final NAND to output
			
			*val += 1;
		}
	}
	
	println!("part 1: {}", pulses[0] * pulses[1]);
	
	// We assume the periods of the counters are coprime.
	// The output pulses low when all counters pulse hi in one step, so:
	println!("part 2: {}", ctr_moduli.iter().map(|n| *n as u64).product::<u64>());
}
