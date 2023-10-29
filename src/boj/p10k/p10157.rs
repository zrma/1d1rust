use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10157(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (col, row) = read_values!(read_line(reader), i32, i32);

    let k = read_line(reader).parse::<i32>().unwrap();

    if row * col < k {
        write!(writer, "0").unwrap();
        return;
    }

    let mut x = 1;
    let mut y = 1;
    let mut k0 = k - 1;
    let mut col0 = col - 1;
    let mut row0 = row;
    let mut dir = Direction::Up;

    loop {
        match dir {
            Direction::Up => {
                if k0 < row0 {
                    write!(writer, "{} {}", x, y + k0).unwrap();
                    return;
                }
                x += 1;
                y += row0 - 1;
                k0 -= row0;
                row0 -= 1;
                dir = turn(dir);
            }
            Direction::Right => {
                if k0 < col0 {
                    write!(writer, "{} {}", x + k0, y).unwrap();
                    return;
                }
                x += col0 - 1;
                y -= 1;
                k0 -= col0;
                col0 -= 1;
                dir = turn(dir);
            }
            Direction::Down => {
                if k0 < row0 {
                    write!(writer, "{} {}", x, y - k0).unwrap();
                    return;
                }
                x -= 1;
                y -= row0 - 1;
                k0 -= row0;
                row0 -= 1;
                dir = turn(dir);
            }
            Direction::Left => {
                if k0 < col0 {
                    write!(writer, "{} {}", x - k0, y).unwrap();
                    return;
                }
                x -= col0 - 1;
                y += 1;
                k0 -= col0;
                col0 -= 1;
                dir = turn(dir);
            }
        }
    }
}

enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

fn turn(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

// https://www.acmicpc.net/problem/10157
// 자리배정
#[test]
fn test_solve10157() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 6
11"
            .to_string(),
            want: "6 6".to_string(),
        },
        TestData {
            s: "7 6
87"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "100 100
3000"
                .to_string(),
            want: "9 64".to_string(),
        },
        TestData {
            s: "7 6
1"
            .to_string(),
            want: "1 1".to_string(),
        },
        TestData {
            s: "7 6
6"
            .to_string(),
            want: "1 6".to_string(),
        },
        TestData {
            s: "7 6
7"
            .to_string(),
            want: "2 6".to_string(),
        },
        TestData {
            s: "7 6
12"
            .to_string(),
            want: "7 6".to_string(),
        },
        TestData {
            s: "7 6
13"
            .to_string(),
            want: "7 5".to_string(),
        },
        TestData {
            s: "7 6
17"
            .to_string(),
            want: "7 1".to_string(),
        },
        TestData {
            s: "7 6
18"
            .to_string(),
            want: "6 1".to_string(),
        },
        TestData {
            s: "10000 10000
99999999"
                .to_string(),
            want: "5001 5001".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10157(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
