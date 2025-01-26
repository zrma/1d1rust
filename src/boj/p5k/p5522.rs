use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5522(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut sum = 0;
    for _ in 0..5 {
        let score: i32 = read_value(read_line(reader));
        sum += score;
    }
    writeln!(writer, "{}", sum).unwrap();
}

// https://www.acmicpc.net/problem/5522
// 카드 게임
#[test]
fn test_solve5522() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "1
2
3
4
5"
            .to_string(),
            want: "15".to_string(),
        },
        TestCase {
            s: "1
1
1
1
1"
            .to_string(),
            want: "5".to_string(),
        },
        TestCase {
            s: "10000
10000
10000
10000
10000"
                .to_string(),
            want: "50000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5522(&mut reader, &mut writer);

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
