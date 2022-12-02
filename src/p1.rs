const INPUT: &str = include_str!("inputs/input_1");

fn solve_one() -> u32 {
    let lines = INPUT.split("\n\n");

    let mut ans = 0;

    for line in lines {
        let s = line.lines().map(|l| l.parse::<u32>().unwrap()).sum();
        if s > ans {
            ans = s;
        }
    }
    ans
}

fn solve_two() -> u32 {
    let lines = INPUT.split("\n\n");

    let mut sums = Vec::new();

    for line in lines {
        let s: u32 = line.lines().map(|l| l.parse::<u32>().unwrap()).sum();
        sums.push(s);
    }
    sums.sort_by(|a, b| b.cmp(a));
    sums[0] + sums[1] + sums[2]
}
