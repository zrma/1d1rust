use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25629(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value::<usize>(read_line(reader));
    let numbers = read_n_values::<i32>(reader, n);

    let (odd_cnt, even_cnt) = numbers
        .iter()
        .fold((0, 0), |(odd, even), &num| match num % 2 {
            0 => (odd, even + 1),
            _ => (odd + 1, even),
        });

    let ans = if odd_cnt == even_cnt || odd_cnt == even_cnt + 1 {
        1
    } else {
        0
    };

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25629
// 홀짝 수열
#[test]
fn test_solve25629() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "3
2 1 1"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3
4 2 3"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "5
4 1 6 3 5"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve25629(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
