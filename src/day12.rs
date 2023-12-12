use std::collections::HashMap;

struct Solver<'a> {
	ng_mask: u128,
	pot_mask: u128,
	groups: &'a [u8],
	cache: HashMap<(u32, u32), u64>,
	debug: bool,
}

impl<'a> Solver<'a> {
	fn count_arrangements_from(&mut self, col_off: u32, grp_off: u32) -> u64 {
		let pot_mask = self.pot_mask >> col_off;
		let intv_len = pot_mask.trailing_ones();
		if self.debug {
			println!("intv_len: {}", intv_len);
		}
		let intv_mask = (1u128 << intv_len) - 1;
		let ng_mask = (self.ng_mask >> col_off) & intv_mask;
		
		let mut arr_cnt = 0;
		let group_len = self.groups[grp_off as usize] as u32;
		if group_len <= intv_len {
			let max_offset = ng_mask.trailing_zeros().min(intv_len - group_len);
			for offset in 0..=max_offset {
				if ng_mask >> (offset + group_len) & 1 == 1 { // mandatory NG that would extend the group
					continue;
				}
				arr_cnt += self.count_arrangements_from_cached(col_off + offset + group_len + 1, grp_off + 1);
			}
		}
		if ng_mask == 0 {
			arr_cnt += self.count_arrangements_from_cached(col_off + intv_len + 1, grp_off);
		}
		arr_cnt
	}
	
	fn count_arrangements_from_cached(&mut self, mut col_off: u32, grp_off: u32) -> u64 {
		if grp_off == self.groups.len() as u32 {
			return if self.ng_mask >> col_off == 0 { 1 } else { 0 };
		}
		let pot_mask = self.pot_mask >> col_off;
		if pot_mask == 0 { return 0; }
		col_off += pot_mask.trailing_zeros();
		
		if let Some(res) = self.cache.get(&(col_off, grp_off)) {
			*res
		} else {
			if self.debug {
				println!("count_arrangements_from({}, {})", col_off, grp_off);
			}
			let res = self.count_arrangements_from(col_off, grp_off);
			if self.debug {
				println!("=> {}", res);
			}
			self.cache.insert((col_off, grp_off), res);
			res
		}
	}
	
	pub fn count_arrangements(ng_mask: u128, na_mask: u128, groups: &[u8], debug: bool) -> u64 {
		if debug {
			println!("count_arrangements(");
			println!("  ng_mask: {:0128b},", ng_mask);
			println!("  na_mask: {:0128b},", na_mask);
			println!("  groups: {:?},", groups);
			println!(")");
		}
		Solver {
			ng_mask,
			pot_mask: ng_mask | na_mask,
			groups,
			cache: HashMap::new(),
			debug
		}.count_arrangements_from_cached(0, 0)
	}
}

fn main() {
	let input = include_str!("../inputs/day12.txt");
	let mut records = vec![];
	for line in input.lines() {
		let mut ng_mask = 0;
		let mut na_mask = 0;
		let mut row_len = None;
		for (i, c) in line.char_indices() {
			match c {
				'.' => {},
				'#' => ng_mask |= 1u128 << i,
				'?' => na_mask |= 1u128 << i,
				' ' => {
					row_len = Some(i);
					break;
				},
				_ => unreachable!()
			}
		}
		let row_len = row_len.unwrap();
		let mut groups = vec![];
		for group_str in line[row_len+1..].split(',') {
			groups.push(group_str.parse::<u8>().unwrap());
		}
		records.push((ng_mask, na_mask, groups, row_len));
	}
	
	let mut arr_cnt_sum = 0;
	for (ng_mask, na_mask, groups, _) in records.iter() {
		arr_cnt_sum += Solver::count_arrangements(*ng_mask, *na_mask, groups, false);
	}
	println!("part 1: {}", arr_cnt_sum);
	
	let mut arr_cnt_sum = 0;
	for (ng_mask, na_mask, groups, row_len) in records {
		assert!(row_len*5 + 4 <= 128);
		let mut ng_mask_ext = ng_mask;
		let mut na_mask_ext = na_mask;
		let mut groups_ext = groups.clone();
		for _ in 0..4 {
			ng_mask_ext = ng_mask_ext << (row_len+1) | ng_mask;
			na_mask_ext = na_mask_ext << (row_len+1) | 1u128 << row_len | na_mask;
			groups_ext.extend_from_slice(&groups);
		}
		
		arr_cnt_sum += Solver::count_arrangements(ng_mask_ext, na_mask_ext, &groups_ext, false);
	}
	println!("part 2: {}", arr_cnt_sum);
}