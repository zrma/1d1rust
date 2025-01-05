use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17144(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r, c, t) = read_values_as!(read_line(reader), usize, usize, usize);
    let mut current_room: Vec<Vec<i32>> = (0..r).map(|_| read_n_values(reader, c)).collect();

    let purifier_positions: Vec<usize> = (0..r).filter(|&y| current_room[y][0] == -1).collect();
    let purifier_top = purifier_positions[0];
    let purifier_bottom = purifier_positions[1];

    for _ in 0..t {
        current_room = spread(&current_room);
        purify(&mut current_room, purifier_top, purifier_bottom);
    }

    let ans: i32 = current_room
        .iter()
        .flatten()
        .filter(|&&cell| cell > 0)
        .sum();
    writeln!(writer, "{}", ans).unwrap();
}

struct MoveParams {
    y: usize,
    x: usize,
    dy: isize,
    dx: isize,
    limit_y: usize,
    limit_x: usize,
}

fn move_purifier(room: &mut [Vec<i32>], params: &MoveParams, mut prev: i32) -> i32 {
    let mut y = params.y;
    let mut x = params.x;

    while (y != params.limit_y || params.dy == 0) && (x != params.limit_x || params.dx == 0) {
        std::mem::swap(&mut room[y][x], &mut prev);

        if let Some(new_y) = y.checked_add_signed(params.dy) {
            y = new_y;
        } else {
            break;
        }

        if let Some(new_x) = x.checked_add_signed(params.dx) {
            x = new_x;
        } else {
            break;
        }
    }
    prev
}

fn execute_purifier_movements(room: &mut [Vec<i32>], movements: &[MoveParams]) {
    let mut prev = 0;
    for params in movements {
        prev = move_purifier(room, params, prev);
    }
}

fn purify(room: &mut [Vec<i32>], purifier_top: usize, purifier_bottom: usize) {
    let c = room[0].len();
    let r = room.len();

    let top_movements = [
        MoveParams {
            y: purifier_top,
            x: 1,
            dy: 0,
            dx: 1,
            limit_y: purifier_top,
            limit_x: c - 1,
        },
        MoveParams {
            y: purifier_top,
            x: c - 1,
            dy: -1,
            dx: 0,
            limit_y: 0,
            limit_x: c - 1,
        },
        MoveParams {
            y: 0,
            x: c - 1,
            dy: 0,
            dx: -1,
            limit_y: 0,
            limit_x: 0,
        },
        MoveParams {
            y: 0,
            x: 0,
            dy: 1,
            dx: 0,
            limit_y: purifier_top,
            limit_x: 0,
        },
    ];

    let bottom_movements = [
        MoveParams {
            y: purifier_bottom,
            x: 1,
            dy: 0,
            dx: 1,
            limit_y: purifier_bottom,
            limit_x: c - 1,
        },
        MoveParams {
            y: purifier_bottom,
            x: c - 1,
            dy: 1,
            dx: 0,
            limit_y: r - 1,
            limit_x: c - 1,
        },
        MoveParams {
            y: r - 1,
            x: c - 1,
            dy: 0,
            dx: -1,
            limit_y: r - 1,
            limit_x: 0,
        },
        MoveParams {
            y: r - 1,
            x: 0,
            dy: -1,
            dx: 0,
            limit_y: purifier_bottom,
            limit_x: 0,
        },
    ];

    execute_purifier_movements(room, &top_movements);
    execute_purifier_movements(room, &bottom_movements);
}

fn spread_dust(y: usize, x: usize, room: &[Vec<i32>], new_room: &mut [Vec<i32>]) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let amount = room[y][x] / 5;
    let mut spread_count = 0;
    let r = room.len();
    let c = room[0].len();

    for &(dy, dx) in &directions {
        if let (Some(ny), Some(nx)) = (y.checked_add_signed(dy), x.checked_add_signed(dx)) {
            if ny < r && nx < c && room[ny][nx] != -1 {
                new_room[ny][nx] += amount;
                spread_count += 1;
            }
        }
    }

    new_room[y][x] -= amount * spread_count;
}

fn spread(room: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_room = room.to_owned();
    let r = room.len();
    let c = room[0].len();
    for y in 0..r {
        for x in 0..c {
            if room[y][x] > 0 {
                spread_dust(y, x, room, &mut new_room);
            }
        }
    }
    new_room
}

// https://www.acmicpc.net/problem/17144
// 미세먼지 안녕!
#[test]
fn test_solve17144() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "7 8 1
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "188".to_string(),
        },
        TestData {
            s: "7 8 2
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "188".to_string(),
        },
        TestData {
            s: "7 8 3
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "186".to_string(),
        },
        TestData {
            s: "7 8 4
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "178".to_string(),
        },
        TestData {
            s: "7 8 5
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "172".to_string(),
        },
        TestData {
            s: "7 8 20
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "71".to_string(),
        },
        TestData {
            s: "7 8 30
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "52".to_string(),
        },
        TestData {
            s: "7 8 50
0 0 0 0 0 0 0 9
0 0 0 0 3 0 0 8
-1 0 5 0 0 0 22 0
-1 8 0 0 0 0 0 0
0 0 0 0 0 10 43 0
0 0 5 0 15 0 0 0
0 0 40 0 0 0 20 0"
                .to_string(),
            want: "46".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17144(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
