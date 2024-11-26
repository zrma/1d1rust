use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6322(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut i = 1;
    loop {
        let s = read_line(reader);
        let (a, b, c): (i32, i32, i32) = {
            let mut s = s.split_whitespace();
            (
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
                s.next().unwrap().parse().unwrap(),
            )
        };

        if a == 0 && b == 0 && c == 0 {
            break;
        }

        let fa = a as f64;
        let fb = b as f64;
        let fc = c as f64;

        writeln!(writer, "Triangle #{}", i).expect("Failed to write");

        match (a, b, c) {
            (-1, _, _) => {
                if c * c - b * b <= 0 {
                    writeln!(writer, "Impossible.").expect("Failed to write");
                } else {
                    writeln!(writer, "a = {:.3}", (fc * fc - fb * fb).sqrt())
                        .expect("Failed to write");
                }
            }
            (_, -1, _) => {
                if c * c - a * a <= 0 {
                    writeln!(writer, "Impossible.").expect("Failed to write");
                } else {
                    writeln!(writer, "b = {:.3}", (fc * fc - fa * fa).sqrt())
                        .expect("Failed to write");
                }
            }
            _ => {
                writeln!(writer, "c = {:.3}", (fa * fa + fb * fb).sqrt()).expect("Failed to write")
            }
        }

        writeln!(writer).expect("Failed to write");

        i += 1;
    }
}

// https://www.acmicpc.net/problem/6322
// 직각 삼각형의 두 변
#[test]
fn test_solve6322() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 4 -1
-1 2 7
5 -1 3
0 0 0"
                .to_string(),
            want: "Triangle #1
c = 5.000

Triangle #2
a = 6.708

Triangle #3
Impossible.

"
            .to_string(),
        },
        TestData {
            s: "3 4 -1
0 0 0 "
                .to_string(),
            want: "Triangle #1
c = 5.000

"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6322(&mut reader, &mut writer);

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
