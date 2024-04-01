use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23841(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);
    let mut board = Vec::with_capacity(n);

    for _ in 0..n {
        let s = read_line(reader).chars().collect::<Vec<_>>();
        board.push(s);
    }

    for row in &mut board {
        for idx in 0..m / 2 {
            let mirror_idx = m - idx - 1;
            if row[idx] == '.' || row[mirror_idx] == '.' {
                row[idx] = std::cmp::max(row[idx], row[mirror_idx]);
                row[mirror_idx] = row[idx];
            }
        }
    }

    let answer = board
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    write!(writer, "{}", answer).expect("Failed to write");
}

// https://www.acmicpc.net/problem/23841
// noinspection SpellCheckingInspection
// 데칼코마니
#[test]
fn test_solve23841() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 6
G..R..
..B...
Y....."
                .to_string(),
            want: "G.RR.G
..BB..
Y....Y"
                .to_string(),
        },
        TestData {
            s: "1 2
G."
            .to_string(),
            want: "GG".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve23841(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
