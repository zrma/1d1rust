use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23375(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (x, y) = read_values_as!(read_line(reader), i32, i32);

    let r = read_line(reader).parse::<i32>().unwrap();

    let (x1, y1) = (x - r, y + r);
    let (x2, y2) = (x + r, y + r);
    let (x3, y3) = (x + r, y - r);
    let (x4, y4) = (x - r, y - r);

    writeln!(writer, "{} {}", x1, y1).expect("Failed to write");
    writeln!(writer, "{} {}", x2, y2).expect("Failed to write");
    writeln!(writer, "{} {}", x3, y3).expect("Failed to write");
    writeln!(writer, "{} {}", x4, y4).expect("Failed to write");
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

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
