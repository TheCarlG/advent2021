use advent2021::common;

const SIZE: usize = 5;
const BOARD_SIZE: usize = SIZE.pow(2);

const H_MATCH: u32 = 0x1F;
const V_MATCH: u32 = 0x108421;

fn parse(mut lines: Vec<String>) -> (Vec<i32>, Vec<[i32; 25]>) {
    let drawn = lines
        .remove(0)
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .into_iter()
        .collect::<Vec<i32>>();

    let (boards, _) = lines.iter().skip(1).fold(
        (vec![[0_i32; BOARD_SIZE]], 0_usize),
        |(mut boards, mut y), row| {
            if row.is_empty() {
                y = 0;
                boards.push([0_i32; BOARD_SIZE]);
            } else {
                let i = boards.len() - 1;
                row.split(' ')
                    .filter(|v| !v.is_empty())
                    .map(|n| n.parse::<i32>().unwrap())
                    .enumerate()
                    .for_each(|(x, n)| {
                        boards[i][(SIZE * y + x)] = n;
                    });
                y += 1;
            }
            (boards, y)
        },
    );

    (drawn, boards)
}

fn part1(drawn: &[i32], boards: &[[i32; 25]]) -> i32 {
    let mut marked = vec![0_u32];
    marked.resize(boards.len(), 0);
    let mut w_board = 0;
    let mut w_num = 0;
    'outer: for num in drawn.iter() {
        for (i, board) in boards.iter().enumerate() {
            if let Some(idx) = board.iter().position(|&v| v == *num) {
                marked[i] |= 1 << (idx);
            }
            for j in 0..BOARD_SIZE {
                let tmp = marked[i] >> j;
                if j % SIZE == 0 && (tmp & H_MATCH) == H_MATCH {
                    w_board = i;
                    w_num = *num;
                    break 'outer;
                }
                if (V_MATCH & tmp) == V_MATCH {
                    w_board = i;
                    w_num = *num;
                    break 'outer;
                }
            }
        }
    }

    let p = marked[w_board];
    let mut sum = 0;
    for i in 0..BOARD_SIZE {
        if (p >> i) & 1 != 1 {
            sum += boards[w_board][i];
        }
    }
    sum * w_num
}

fn part2(_drawn: &[i32], _boards: &[[i32; 25]]) -> u32 {
    0
}

fn main() {
    common::time_func(|| {
        let lines = common::read_input::<String>("input/day04.data");
        let (drawn, boards) = parse(lines);

        println!("Part01: {}", part1(&drawn, &boards));
        println!("Part02: {}", part2(&drawn, &boards));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = common::read_input::<String>("input/day04.test");
        let (drawn, boards) = parse(lines);

        assert_eq!(part1(&drawn, &boards), 4512);

        let lines = common::read_input::<String>("input/day04-2.test");
        let (drawn, boards) = parse(lines);
        assert_eq!(part1(&drawn, &boards), 2772);
    }

    // #[test]
    // fn test_part2() {
    //     let lines = common::read_input::<String>("input/day04.test");
    //     let (drawn, boards) = parse(lines);
    //     assert_eq!(part2(&drawn, &boards), 0);
    // }
}
