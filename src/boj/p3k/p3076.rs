use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3076(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r, c) = read_values_as!(read_line(reader), usize, usize);
    let (a, b) = read_values_as!(read_line(reader), usize, usize);

    for i in 0..r * a {
        for j in 0..c * b {
            if (i / a + j / b) % 2 == 0 {
                write!(writer, "X").expect("Failed to write");
            } else {
                write!(writer, ".").expect("Failed to write");
            }
        }
        writeln!(writer).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/3076
// 상근이의 체스판
#[test]
fn test_solve3076() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2 4
2 2"
            .to_string(),
            want: "XX..XX..
XX..XX..
..XX..XX
..XX..XX
"
            .to_string(),
        },
        TestData {
            s: "5 5
2 3"
            .to_string(),
            want: "XXX...XXX...XXX
XXX...XXX...XXX
...XXX...XXX...
...XXX...XXX...
XXX...XXX...XXX
XXX...XXX...XXX
...XXX...XXX...
...XXX...XXX...
XXX...XXX...XXX
XXX...XXX...XXX
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3076(&mut reader, &mut writer);

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
