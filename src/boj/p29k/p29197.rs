use crate::read_values;
use crate::utils::io::{read_line, read_value};
use num::integer::gcd;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve29197(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut set = std::collections::HashSet::new();

    for _ in 0..n {
        let (x, y) = read_values!(read_line(reader), i64, i64);

        let r = Rational::new(y, x);
        set.insert(r);
    }

    write!(writer, "{}", set.len()).unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    fn new(numerator: i64, denominator: i64) -> Self {
        let gcd = gcd(numerator, denominator);
        Self {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        }
    }
}

// https://www.acmicpc.net/problem/29197
// 아침 태권도
#[test]
fn test_solve29197() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
1 2
2 4
-2 2
-4 4
-1 0
-3 0
2 -1"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5
1 2
2 4
-2 2
-4 4
-1 0"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve29197(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
