use nom::{
    multi::separated_list1,
    bytes::complete::tag,
    character::complete::{digit1, char},
    sequence::{delimited, separated_pair},
    combinator::{map_res, map, all_consuming},
    IResult, branch::alt
};

#[derive(Debug, Clone, Copy)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}
#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_line(rst: &str) -> IResult<&str, Game> {
    let (rst, id) = delimited(
        tag("Game "),
        map_res(digit1, str::parse),
        tag(": ")
    )(rst)?;
    let (rst, sets) = separated_list1(tag("; "),
        map(separated_list1(tag(", "), separated_pair(
            map_res(digit1, str::parse),
            char(' '),
            alt((
                tag("red"),
                tag("green"),
                tag("blue")
            ))
        )), |parts: Vec<(u32, &str)>| {
            let mut set = Set { r: 0, g: 0, b: 0 };
            for (cnt, col) in parts {
                match col {
                    "red" => set.r += cnt,
                    "green" => set.g += cnt,
                    "blue" => set.b += cnt,
                    _ => unreachable!()
                }
            }
            set
        })
    )(rst)?;
    Ok((rst, Game { id, sets }))
}

fn main() {
	let input = include_str!("../inputs/day2.txt");
    let mut games = vec![];
    for line in input.lines() {
        games.push(all_consuming(parse_line)(line).unwrap().1);
    }

    let mut id_sum = 0;
    for game in &games {
        if game.sets.iter().all(|s| s.r <= 12 && s.g <= 13 && s.b <= 14) {
            id_sum += game.id;
        }
    }
    println!("part 1: {}", id_sum);

    let mut power_sum = 0;
    for game in games {
        let min_cubes = game.sets.iter().copied().reduce(|s1, s2|
            Set { r: s1.r.max(s2.r), g: s1.g.max(s2.g), b: s1.b.max(s2.b) }
        ).unwrap();
        let power = min_cubes.r * min_cubes.g * min_cubes.b;
        power_sum += power;
    }
    println!("part 2: {}", power_sum);
}