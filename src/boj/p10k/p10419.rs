use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10419(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_cases = read_value(read_line(reader));
    let ans = (0..num_cases)
        .map(|_| {
            let d = read_value(read_line(reader));
            let mut t = 0;
            while t * (t + 1) <= d {
                t += 1;
            }
            (t - 1).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/10419
// 지각
#[test]
fn test_solve10419() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
1
2
5
6
7"
            .to_string(),
            want: "0
1
1
2
2"
            .to_string(),
        },
        TestData {
            s: "1
12"
            .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10419(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
