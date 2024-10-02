use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9085(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t: usize = read_value(read_line(reader));
    let mut res = Vec::with_capacity(t);

    for _ in 0..t {
        let n = read_value(read_line(reader));
        let sum: i32 = read_n_values(reader, n).iter().sum();
        res.push(sum.to_string());
    }

    write!(writer, "{}", res.join("\n")).expect("Failed to write");
}

// https://www.acmicpc.net/problem/9085
// 더하기
#[test]
fn test_solve9085() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
5
1 1 1 1 1
7
1 2 3 4 5 6 7"
                .to_string(),
            want: "5
28"
            .to_string(),
        },
        TestData {
            s: "1
4
1 2 3 4"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve9085(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
