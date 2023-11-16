use crate::read_values;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9723(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value(read_line(reader));
    for i in 0..t {
        let (a, b, c) = read_values!(read_line(reader), i32, i32, i32);
        let res = if is_pythagorean_triplet(a, b, c) {
            "YES"
        } else {
            "NO"
        };

        writeln!(writer, "Case #{}: {}", i + 1, res).unwrap();
    }
}

fn is_pythagorean_triplet(a: i32, b: i32, c: i32) -> bool {
    let a2 = a * a;
    let b2 = b * b;
    let c2 = c * c;
    a2 + b2 == c2 || a2 + c2 == b2 || b2 + c2 == a2
}

// https://www.acmicpc.net/problem/9723
// Right Triangle
#[test]
fn test_solve9723() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10
20 16 12
5 3 4
15 12 9
12 5 13
12 13 5
28 82 46
43 96 92
3 4 5
13 5 12
6 10 8"
                .to_string(),
            want: "Case #1: YES
Case #2: YES
Case #3: YES
Case #4: YES
Case #5: YES
Case #6: NO
Case #7: NO
Case #8: YES
Case #9: YES
Case #10: YES
"
            .to_string(),
        },
        TestData {
            s: "1
1 1 1"
                .to_string(),
            want: "Case #1: NO
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9723(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
