const INPUT: &str = include_str!("inputs/input_2");

fn solve_one() -> i32 {
    INPUT
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" ").unwrap();
            let score = match (a, b) {
                ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                _ => 0,
            };
            let result = match b {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };
            score + result
        })
        .sum()
}

fn solve_two() -> i32 {
    INPUT
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" ").unwrap();
            let score = match (a, b) {
                ("A", "Y") | ("B", "X") | ("C", "Z") => 1,
                ("A", "Z") | ("B", "Y") | ("C", "X") => 2,
                ("A", "X") | ("B", "Z") | ("C", "Y") => 3,
                _ => 0,
            };
            let result = match b {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };
            score + result
        })
        .sum()
}
