use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24723(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _: i32 = read_line(reader).parse().unwrap();
    let _: i32 = read_line(reader).parse().unwrap();

    writeln!(writer, "90").unwrap();
}

// https://www.acmicpc.net/problem/24723
// Изгороди
#[test]
fn test_solve24723() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
90"
            .to_string(),
            want: "90".to_string(),
        },
        TestData {
            s: "123
180"
            .to_string(),
            want: "90".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve24723(&mut reader, &mut writer);

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
