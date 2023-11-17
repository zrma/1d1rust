use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1987(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r, c) = read_values!(read_line(reader), usize, usize);
    let board = read_board(reader, r, c);

    let mut visited = vec![false; 26];
    let ans = dfs(&mut visited, &board, 0, 0, 1);

    write!(writer, "{}", ans).unwrap();
}

fn read_board(reader: &mut impl BufRead, r: usize, c: usize) -> Vec<Vec<char>> {
    (0..r)
        .map(|_| read_line(reader).chars().take(c).collect())
        .collect()
}

fn dfs(visited: &mut Vec<bool>, board: &[Vec<char>], x: usize, y: usize, dist: i32) -> i32 {
    let i = (board[x][y] as u8 - b'A') as usize;
    if visited[i] {
        return dist - 1;
    }
    visited[i] = true;

    let mut max_dist = dist;

    let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in directions {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0
            && new_x < board.len() as isize
            && new_y >= 0
            && new_y < board[0].len() as isize
        {
            max_dist = max_dist.max(dfs(
                visited,
                board,
                new_x as usize,
                new_y as usize,
                dist + 1,
            ));
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
