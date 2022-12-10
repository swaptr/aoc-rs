use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/input_3");
fn solve_one() -> u64 {
    INPUT
        .lines()
        .map(|line| {
            let (split_one, split_two) = line.split_at(line.len() / 2);
            let set_one: HashSet<char> = split_one.chars().collect();
            let set_two: HashSet<char> = split_two.chars().collect();
            (match *set_one.intersection(&set_two).next().unwrap() as u8 {
                c @ b'a'..=b'z' => 1 + c - b'a',
                c @ b'A'..=b'Z' => 27 + c - b'A',
                _ => unreachable!(),
            }) as u64
        })
        .sum()
}

fn solve_two() -> u64 {
    INPUT
        .lines()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .tuples()
        .map(|(a, b, c)| {
            let d = &(&a & &b) & &c;
            (match d.into_iter().next().unwrap() as u8 {
                ch @ b'a'..=b'z' => 1 + ch - b'a',
                ch @ b'A'..=b'Z' => 27 + ch - b'A',
                _ => unreachable!(),
            }) as u64
        })
        .sum()
}
