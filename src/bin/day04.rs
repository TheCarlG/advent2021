use advent2021::common;

const DAY: &str = "day04";

const SIZE: usize = 5;
const BOARD_SIZE: usize = SIZE.pow(2);

const H_MATCH: u32 = 0x1F;
const V_MATCH: u32 = 0x108421;

fn parse(mut lines: Vec<String>) -> (Vec<i32>, Vec<([i32; BOARD_SIZE], u32)>) {
    let drawn = lines
        .remove(0)
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .into_iter()
        .collect::<Vec<i32>>();

    let (boards, _) = lines.iter().skip(1).fold(
        (vec![([0_i32; BOARD_SIZE], 0_u32)], 0_usize),
        |(mut boards, mut y), row| {
            if row.is_empty() {
                y = 0;
                boards.push(([0_i32; BOARD_SIZE], 0_u32));
            } else {
                let i = boards.len() - 1;
                row.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|n| n.parse::<i32>().unwrap())
                    .enumerate()
                    .for_each(|(x, n)| {
                        boards[i].0[(SIZE * y + x)] = n;
                    });
                y += 1;
            }
            (boards, y)
        },
    );

    (drawn, boards)
}

fn play(drawn: &[i32], mut boards: Vec<([i32; BOARD_SIZE], u32)>) -> (i32, i32) {
    let winners = drawn.iter().fold(Vec::new(), |mut winners, num| {
        boards = boards
            .iter()
            .map(|&(mut board)| {
                if let Some(idx) = board.0.iter().position(|&v| v == *num) {
                    board.1 |= 1 << idx;
                }
                board
            })
            .filter(|board| {
                for j in 0..BOARD_SIZE {
                    let tmp = board.1 >> j;
                    if (j % SIZE == 0 && (tmp & H_MATCH) == H_MATCH) || (V_MATCH & tmp) == V_MATCH {
                        winners.push((num, (board.0, board.1)));
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<([i32; BOARD_SIZE], u32)>>();
        winners
    });

    let (first_n, first_b) = winners.first().unwrap();
    let (last_n, last_b) = winners.last().unwrap();

    let mut first_s = 0;
    let mut last_s = 0;
    for i in 0..BOARD_SIZE {
        if (first_b.1 >> i) & 1 != 1 {
            first_s += first_b.0[i];
        }
        if (last_b.1 >> i) & 1 != 1 {
            last_s += last_b.0[i];
        }
    }

    (first_s * **first_n, last_s * **last_n)
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>(DAY, false);
        let (drawn, boards) = parse(lines);

        let (part1, part2) = play(&drawn, boards);
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>(DAY, true);
        let (drawn, boards) = parse(lines);

        assert_eq!(play(&drawn, boards).0, 4512);

        let lines = common::read_input::<String>("day04-2", true);
        let (drawn, boards) = parse(lines);
        assert_eq!(play(&drawn, boards).0, 2772);
    }

    #[test]
    fn test_part2() {
        let lines = common::read_input::<String>(DAY, true);
        let (drawn, boards) = parse(lines);
        assert_eq!(play(&drawn, boards).1, 1924);
    }
}
