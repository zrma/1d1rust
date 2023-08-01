use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11179(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<i32>().unwrap();

    let mut ans = 0;
    let mut n = n;
    while n > 0 {
        ans = ans * 2 + n % 2;
        n /= 2;
    }

    write!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/11179
// 2진수 뒤집기
#[test]
fn test_solve11179() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "13".to_string(),
            want: "11".to_string(),
        },
        TestData {
            s: "47".to_string(),
            want: "61".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11179(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
