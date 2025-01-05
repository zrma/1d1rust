use crate::read_values_as;
use crate::utils::io::read_line;
use std::cmp::Ordering::Less;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10817(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

    let mid = match (a.cmp(&b), b.cmp(&c), c.cmp(&a)) {
        (Less, Less, _) => b,
        (_, Less, Less) => c,
        (Less, _, Less) => a,
        (Less, _, _) => c,
        (_, Less, _) => a,
        (_, _, Less) => b,
        (_, _, _) => b,
    };

    writeln!(writer, "{}", mid).unwrap();
}

// https://www.acmicpc.net/problem/10817
// 세 수
#[test]
fn test_solve10817() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "20 30 10".to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "30 30 10".to_string(),
            want: "30".to_string(),
        },
        TestData {
            s: "40 40 40".to_string(),
            want: "40".to_string(),
        },
        TestData {
            s: "20 10 10".to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10817(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
