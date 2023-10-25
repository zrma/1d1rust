use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23375(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (x, y) = {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    };

    let r = {
        let s = read_line(reader);
        s.parse::<i32>().unwrap()
    };

    let (x1, y1) = (x - r, y + r);
    let (x2, y2) = (x + r, y + r);
    let (x3, y3) = (x + r, y - r);
    let (x4, y4) = (x - r, y - r);

    writeln!(writer, "{} {}", x1, y1).unwrap();
    writeln!(writer, "{} {}", x2, y2).unwrap();
    writeln!(writer, "{} {}", x3, y3).unwrap();
    writeln!(writer, "{} {}", x4, y4).unwrap();
}

// https://www.acmicpc.net/problem/23375
// Arm Coordination
#[test]
fn test_solve23375() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "-3 6
5"
            .to_string(),
            want: "-8 11
2 11
2 1
-8 1
"
            .to_string(),
        },
        TestData {
            s: "0 0
10"
            .to_string(),
            want: "-10 10
10 10
10 -10
-10 -10
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23375(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
