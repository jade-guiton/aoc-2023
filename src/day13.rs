#![feature(let_chains)]

#[derive(Clone, Debug)]
struct Pattern {
	width: usize,
	lines: Vec<u32>,
}
impl Pattern {
    fn empty() -> Self {
        Pattern { width: 0, lines: vec![] }
    }
    fn transpose(&self) -> Pattern {
        let mut lines = vec![0; self.width];
        for (line_idx, mut line) in self.lines.iter().copied().enumerate() {
            for col in 0..self.width {
                lines[col] |= (line & 1) << line_idx;
                line >>= 1;
            }
        }
        Pattern { width: self.lines.len(), lines }
    }
    fn is_hor_refl(&self, line_pos: usize, smudged: bool) -> bool {
        let mut smudges = 0;
        for dist in 0..line_pos.min(self.lines.len()-line_pos) {
            smudges += (self.lines[line_pos-1-dist] ^ self.lines[line_pos+dist]).count_ones();
        }
        smudges == smudged as u32
    }
    fn find_hor_refl(&self, smudged: bool) -> Option<usize> {
        (1..self.lines.len()).find(|line_pos| self.is_hor_refl(*line_pos, smudged))
    }
    fn summarize_refl(&self, smudged: bool) -> usize {
        self.find_hor_refl(smudged).map(|x| x*100).or_else(||
            self.transpose().find_hor_refl(smudged)
        ).unwrap()
    }
}

fn main() {
    let input = include_str!("../inputs/day13.txt");
    let mut patterns = vec![Pattern::empty()];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern::empty());
        } else {
            let cur_pat = patterns.last_mut().unwrap();
            if cur_pat.width == 0 { cur_pat.width = line.len(); }
            assert_eq!(cur_pat.width, line.len());
            let mut mask = 0;
            for (i,c) in line.char_indices() {
                mask |= ((c == '#') as u32) << i;
            }
            cur_pat.lines.push(mask);
        }
    }
    println!("part 1: {}", patterns.iter().map(|pat| pat.summarize_refl(false)).sum::<usize>());
    println!("part 2: {}", patterns.iter().map(|pat| pat.summarize_refl(true)).sum::<usize>());
}