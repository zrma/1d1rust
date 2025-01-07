use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5724(reader: &mut impl BufRead, writer: &mut impl Write) {
    let dp: Vec<i32> = (0..=100)
        .scan(0, |sum, i| {
            *sum += i * i;
            Some(*sum)
        })
        .collect();

    loop {
        let n: usize = read_value(read_line(reader));
        if n == 0 {
            break;
        }

        writeln!(writer, "{}", dp[n]).unwrap();
    }
}

// https://www.acmicpc.net/problem/5724
// 파인만
#[test]
fn test_solve5724() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2
1
8
0"
            .to_string(),
            want: "5
1
204
"
            .to_string(),
        },
        TestData {
            s: "3
0"
            .to_string(),
            want: "14
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5724(&mut reader, &mut writer);

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
