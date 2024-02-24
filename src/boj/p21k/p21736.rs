use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21736(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values!(read_line(reader), usize, usize);

    let mut map = vec![vec![Block::Empty; m]; n];
    let mut start = (0, 0);

    let mut line = String::new();
    for (y, row) in map.iter_mut().enumerate() {
        line.clear();
        reader.read_line(&mut line).unwrap();

        for (x, c) in line.trim().chars().enumerate() {
            match c {
                'I' => {
                    start = (y, x);
                    row[x] = Block::Start;
                }
                'O' => row[x] = Block::Empty,
                'X' => row[x] = Block::Wall,
                'P' => row[x] = Block::Person,
                _ => unreachable!(),
            }
        }
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    let mut count = 0;
    while let Some((y, x)) = queue.pop_front() {
        match map[y][x] {
            Block::Wall => {
                continue;
            }
            Block::Person => {
                count += 1;
            }
            _ => {}
        }

        map[y][x] = Block::Wall;

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let nx = match x.checked_add_signed(*dx) {
                Some(v) => v,
                None => continue,
            };
            let ny = match y.checked_add_signed(*dy) {
                Some(v) => v,
                None => continue,
            };
            if nx >= m || ny >= n {
                continue;
            }

            if map[ny][nx] == Block::Wall {
                continue;
            }

            queue.push_back((ny, nx));
        }
    }

    let res = count.to_string();

    write!(writer, "{}", if count == 0 { "TT" } else { &res }).unwrap();
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Block {
    Empty,
    Wall,
    Person,
    Start,
}

// https://www.acmicpc.net/problem/21736
// 헌내기는 친구가 필요해
#[test]
fn test_solve21736() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 5
OOOPO
OIOOX
OOOXP"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 3
IOX
OXP
XPP"
            .to_string(),
            want: "TT".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve21736(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
