use crate::read_values_as;
use crate::utils::io::read_line;
use std::cmp::Ordering::Less;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve7510(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for i in 1..=t {
        let (a, b, c) = {
            let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

            match (a.cmp(&b), b.cmp(&c), c.cmp(&a)) {
                (Less, Less, _) => (a, b, c),
                (_, Less, Less) => (b, c, a),
                (Less, _, Less) => (c, a, b),
                (Less, _, _) => (a, c, b),
                (_, Less, _) => (b, a, c),
                (_, _, Less) => (c, b, a),
                (_, _, _) => (a, b, c),
            }
        };

        writeln!(writer, "Scenario #{}:", i).unwrap();
        if a * a + b * b == c * c {
            writeln!(writer, "yes").unwrap();
        } else {
            writeln!(writer, "no").unwrap();
        }

        if i != t {
            writeln!(writer).unwrap();
        }
    }
}

// https://www.acmicpc.net/problem/7510
// 고급 수학
#[test]
fn test_solve7510() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
36 77 85
40 55 69"
                .to_string(),
            want: "Scenario #1:
yes

Scenario #2:
no
"
            .to_string(),
        },
        TestData {
            s: "1
1 1 1"
                .to_string(),
            want: "Scenario #1:
no
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve7510(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
