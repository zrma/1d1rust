use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13698(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let mut positions = [1, 0, 0, 2];

    for c in line.chars() {
        match c {
            'A' => positions.swap(0, 1),
            'B' => positions.swap(0, 2),
            'C' => positions.swap(0, 3),
            'D' => positions.swap(1, 2),
            'E' => positions.swap(1, 3),
            'F' => positions.swap(2, 3),
            _ => unreachable!(),
        }
    }

    let pos1 = positions.iter().position(|&x| x == 1).unwrap();
    let pos2 = positions.iter().position(|&x| x == 2).unwrap();

    writeln!(writer, "{}", pos1 + 1).unwrap();
    writeln!(writer, "{}", pos2 + 1).unwrap();
}

// https://www.acmicpc.net/problem/13698
// Hawk eyes
#[test]
fn test_solve13698() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "AB".to_string(),
            want: "2
4"
            .to_string(),
        },
        TestData {
            s: "ABCDEF".to_string(),
            want: "4
1"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13698(&mut reader, &mut writer);

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
