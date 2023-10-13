use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2290(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (s, arr) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let s = iter.next().unwrap().parse::<usize>().unwrap();
        let arr = iter
            .next()
            .unwrap()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect::<Vec<_>>();
        (s, arr)
    };

    let mut board = vec![vec![' '; (s + 3) * arr.len()]; 2 * s + 3];

    arr.iter().enumerate().for_each(|(i, &num)| {
        draw_number(num, i, s, &mut board);
    });

    for row in board {
        for ch in row {
            write!(writer, "{}", ch).unwrap();
        }
        writeln!(writer).unwrap();
    }
}

fn draw_number(number: i32, index: usize, size: usize, board: &mut [Vec<char>]) {
    struct Digit {
        top: bool,
        top_left: bool,
        top_right: bool,
        middle: bool,
        bottom_left: bool,
        bottom_right: bool,
        bottom: bool,
    }

    let digits = [
        Digit {
            top: true,
            top_left: true,
            top_right: true,
            middle: false,
            bottom_left: true,
            bottom_right: true,
            bottom: true,
        }, //0
        Digit {
            top: false,
            top_left: false,
            top_right: true,
            middle: false,
            bottom_left: false,
            bottom_right: true,
            bottom: false,
        }, //1
        Digit {
            top: true,
            top_left: false,
            top_right: true,
            middle: true,
            bottom_left: true,
            bottom_right: false,
            bottom: true,
        }, // 2
        Digit {
            top: true,
            top_left: false,
            top_right: true,
            middle: true,
            bottom_left: false,
            bottom_right: true,
            bottom: true,
        }, // 3
        Digit {
            top: false,
            top_left: true,
            top_right: true,
            middle: true,
            bottom_left: false,
            bottom_right: true,
            bottom: false,
        }, // 4
        Digit {
            top: true,
            top_left: true,
            top_right: false,
            middle: true,
            bottom_left: false,
            bottom_right: true,
            bottom: true,
        }, // 5
        Digit {
            top: true,
            top_left: true,
            top_right: false,
            middle: true,
            bottom_left: true,
            bottom_right: true,
            bottom: true,
        }, // 6
        Digit {
            top: true,
            top_left: false,
            top_right: true,
            middle: false,
            bottom_left: false,
            bottom_right: true,
            bottom: false,
        }, // 7
        Digit {
            top: true,
            top_left: true,
            top_right: true,
            middle: true,
            bottom_left: true,
            bottom_right: true,
            bottom: true,
        }, // 8
        Digit {
            top: true,
            top_left: true,
            top_right: true,
            middle: true,
            bottom_left: false,
            bottom_right: true,
            bottom: true,
        }, // 9
    ];

    let digit = &digits[number as usize];

    if digit.top {
        for i in 0..size {
            board[0][index * (size + 3) + i + 1] = '-';
        }
    }

    if digit.top_left {
        for i in 0..size {
            board[i + 1][index * (size + 3)] = '|';
        }
    }

    if digit.top_right {
        for i in 0..size {
            board[i + 1][index * (size + 3) + size + 1] = '|';
        }
    }

    if digit.middle {
        for i in 0..size {
            board[size + 1][index * (size + 3) + i + 1] = '-';
        }
    }

    if digit.bottom_left {
        for i in 0..size {
            board[i + size + 2][index * (size + 3)] = '|';
        }
    }

    if digit.bottom_right {
        for i in 0..size {
            board[i + size + 2][index * (size + 3) + size + 1] = '|';
        }
    }

    if digit.bottom {
        for i in 0..size {
            board[2 * size + 2][index * (size + 3) + i + 1] = '-';
        }
    }
}

// https://www.acmicpc.net/problem/2290
// LCD Test
// noinspection SpellCheckingInspection
#[test]
fn test_solve2290() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 1234567890".to_string(),
            want: "      --   --        --   --   --   --   --   --  \n   |    |    | |  | |    |       | |  | |  | |  | \n   |    |    | |  | |    |       | |  | |  | |  | \n      --   --   --   --   --        --   --       \n   | |       |    |    | |  |    | |  |    | |  | \n   | |       |    |    | |  |    | |  |    | |  | \n      --   --        --   --        --   --   --  \n"
            .to_string(),
        },
        TestData {
            s: "2 24680".to_string(),
            want: " --        --   --   --  \n   | |  | |    |  | |  | \n   | |  | |    |  | |  | \n --   --   --   --       \n|       | |  | |  | |  | \n|       | |  | |  | |  | \n --        --   --   --  \n"
            .to_string(),
        },
        TestData {
            s: "3 24680".to_string(),
            want: " ---         ---   ---   ---  \n    | |   | |     |   | |   | \n    | |   | |     |   | |   | \n    | |   | |     |   | |   | \n ---   ---   ---   ---        \n|         | |   | |   | |   | \n|         | |   | |   | |   | \n|         | |   | |   | |   | \n ---         ---   ---   ---  \n"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2290(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "Failed test case {}", i);
    }
}
