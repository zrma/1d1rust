use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11536(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let mut is_increasing = true;
    let mut is_decreasing = true;

    let mut prev = read_line(reader);
    for _ in 1..n {
        let s = read_line(reader);
        match s.cmp(&prev) {
            std::cmp::Ordering::Less => is_increasing = false,
            std::cmp::Ordering::Greater => is_decreasing = false,
            std::cmp::Ordering::Equal => {}
        }
        prev = s;
    }

    if is_increasing {
        write!(writer, "INCREASING").expect("Failed to write");
    } else if is_decreasing {
        write!(writer, "DECREASING").expect("Failed to write");
    } else {
        write!(writer, "NEITHER").expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/11536
// 줄 세우기
// noinspection SpellCheckingInspection
#[test]
fn test_solve11536() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
JOE
BOB
ANDY
AL
ADAM"
                .to_string(),
            want: "DECREASING".to_string(),
        },
        TestData {
            s: "11
HOPE
ALI
BECKY
JULIE
MEGHAN
LAUREN
MORGAN
CARLI
MEGAN
ALEX
TOBIN"
                .to_string(),
            want: "NEITHER".to_string(),
        },
        TestData {
            s: "4
GEORGE
JOHN
PAUL
RINGO"
                .to_string(),
            want: "INCREASING".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11536(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
