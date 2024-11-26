use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve10886(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let (cute, not_cute) = (0..n).fold((0, 0), |(cute, not_cute), _| {
        match read_value(read_line(reader)) {
            1 => (cute + 1, not_cute),
            _ => (cute, not_cute + 1),
        }
    });

    let result = if cute > not_cute {
        "Junhee is cute!"
    } else {
        "Junhee is not cute!"
    };

    writeln!(writer, "{}", result).expect("write should work");
}

// https://www.acmicpc.net/problem/10886
// noinspection SpellCheckingInspection
// 0 = not cute / 1 = cute
#[test]
fn test_solve10886() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
1
0
0"
            .to_string(),
            want: "Junhee is not cute!".to_string(),
        },
        TestData {
            s: "1
1"
            .to_string(),
            want: "Junhee is cute!".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10886(&mut reader, &mut writer);

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
