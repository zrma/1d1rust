use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4084(reader: &mut impl BufRead, writer: &mut impl Write) {
    loop {
        let (mut a, mut b, mut c, mut d) = read_values_as!(read_line(reader), i64, i64, i64, i64);

        if a == 0 && b == 0 && c == 0 && d == 0 {
            break;
        }

        let mut count = 0;

        while !(a == b && b == c && c == d) {
            (a, b, c, d) = ((a - b).abs(), (b - c).abs(), (c - d).abs(), (d - a).abs());
            count += 1;
        }

        writeln!(writer, "{}", count).expect("writeln! should work");
    }
}

// https://www.acmicpc.net/problem/4084
// noinspection SpellCheckingInspection
// Viva la Diferencia
#[test]
fn test_solve4084() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 3 5 9
4 3 2 1
1 1 1 1
0 0 0 0"
                .to_string(),
            want: "6
4
0
"
            .to_string(),
        },
        TestData {
            s: "1 2 3 4
0 0 0 0"
                .to_string(),
            want: "4
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve4084(&mut reader, &mut writer);

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
