use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6502(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut i = 1;
    loop {
        let s = read_line(reader);
        if s == "0" {
            break;
        }

        let (r, w, h): (i32, i32, i32) = {
            let mut s = s.split_whitespace();
            (
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        };

        if is_fit(r, w, h) {
            writeln!(writer, "Pizza {} fits on the table.", i).expect("Failed to write");
        } else {
            writeln!(writer, "Pizza {} does not fit on the table.", i).expect("Failed to write");
        }

        i += 1;
    }
}

fn is_fit(r: i32, w: i32, h: i32) -> bool {
    (2 * r).pow(2) >= w * w + h * h
}

// https://www.acmicpc.net/problem/6502
// 동혁 피자
#[test]
fn test_solve6502() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "38 40 60
35 20 70
50 60 80
0"
            .to_string(),
            want: "Pizza 1 fits on the table.
Pizza 2 does not fit on the table.
Pizza 3 fits on the table.
"
            .to_string(),
        },
        TestData {
            s: "38 40 60
0"
            .to_string(),
            want: "Pizza 1 fits on the table.
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6502(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
