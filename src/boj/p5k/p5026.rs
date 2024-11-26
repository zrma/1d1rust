use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5026(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_line(reader).parse().unwrap();

    for _ in 0..n {
        let s = read_line(reader);

        if s == "P=NP" {
            writeln!(writer, "skipped").expect("Failed to write");
        } else {
            let mut iter = s.split('+');
            let a: i32 = iter.next().unwrap().parse().unwrap();
            let b: i32 = iter.next().unwrap().parse().unwrap();
            writeln!(writer, "{}", a + b).expect("Failed to write");
        }
    }
}

// https://www.acmicpc.net/problem/5026
// 박사 과정
#[test]
fn test_solve5026() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
2+2
1+2
P=NP
0+0"
            .to_string(),
            want: "4
3
skipped
0
"
            .to_string(),
        },
        TestData {
            s: "1
3+5"
            .to_string(),
            want: "8
"
            .to_string(),
        },
        TestData {
            s: "1
P=NP"
                .to_string(),
            want: "skipped
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5026(&mut reader, &mut writer);

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
