use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3023(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r, c) = read_values_as!(read_line(reader), usize, usize);

    let mut board = vec![vec!['.'; 2 * c]; 2 * r];
    for i in 0..r {
        let s = read_line(reader);
        for (j, ch) in s.chars().enumerate() {
            board[i][j] = ch;
            board[2 * r - 1 - i][j] = ch;
            board[i][2 * c - 1 - j] = ch;
            board[2 * r - 1 - i][2 * c - 1 - j] = ch;
        }
    }

    let (a, b) = read_values_as!(read_line(reader), usize, usize);

    board[a - 1][b - 1] = if board[a - 1][b - 1] == '.' { '#' } else { '.' };

    for row in board {
        for ch in row {
            write!(writer, "{}", ch).unwrap();
        }
        writeln!(writer).unwrap();
    }
}

// https://www.acmicpc.net/problem/3023
// 마술사 이민혁
#[test]
fn test_solve3023() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 2
#.
.#
3 3"
            .to_string(),
            want: "#..#
.##.
.#..
#..#
"
            .to_string(),
        },
        TestData {
            s: "3 3
###
###
###
1 4"
            .to_string(),
            want: "###.##
######
######
######
######
######
"
            .to_string(),
        },
        TestData {
            s: "5 4
#.#.
#.##
#.##
....
.#.#
10 5"
                .to_string(),
            want: "#.#..#.#
#.####.#
#.####.#
........
.#.##.#.
.#.##.#.
........
#.####.#
#.####.#
#.#.##.#
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3023(&mut reader, &mut writer);

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
