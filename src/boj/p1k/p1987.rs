use crate::read_values_as;
use crate::utils::io::read_line;
use std::convert::TryFrom;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1987(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r, c) = read_values_as!(read_line(reader), usize, usize);
    let board = read_board(reader, r, c);

    let mut visited = vec![false; 26];
    let ans = dfs(&mut visited, &board, 0, 0, 1);

    write!(writer, "{}", ans).expect("write! should work");
}

fn read_board(reader: &mut impl BufRead, r: usize, c: usize) -> Vec<Vec<char>> {
    (0..r)
        .map(|_| read_line(reader).chars().take(c).collect())
        .collect()
}

fn dfs(visited: &mut Vec<bool>, board: &[Vec<char>], x: usize, y: usize, dist: i32) -> i32 {
    let c = board[x][y];
    let c_u8 = u8::try_from(c).expect("Character is not a valid ASCII character");

    if !c_u8.is_ascii_uppercase() {
        panic!("Character out of expected range 'A' to 'Z'");
    }

    let value = c_u8
        .checked_sub(b'A')
        .expect("Subtraction underflow: character less than 'A'");

    let i = usize::from(value);

    if visited[i] {
        return dist - 1;
    }
    visited[i] = true;

    let mut max_dist = dist;

    let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in directions {
        let nx = match x.checked_add_signed(dx) {
            Some(v) => v,
            None => continue,
        };
        let ny = match y.checked_add_signed(dy) {
            Some(v) => v,
            None => continue,
        };
        if nx < board.len() && ny < board[0].len() {
            max_dist = max_dist.max(dfs(visited, board, nx, ny, dist + 1));
        }
    }

    visited[i] = false;
    max_dist
}

// https://www.acmicpc.net/problem/1987
// noinspection SpellCheckingInspection
// 알파벳
#[test]
fn test_solve1987() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 4
CAAB
ADCB"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "3 6
HFDFFB
AJHGDH
DGAGEH"
                .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "5 5
IEFCJ
FHFKC
FFALF
HFGCF
HMCHH"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1987(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
