use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23972(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (k, n) = read_values_as!(read_line(reader), u64, u64);

    if n == 1 {
        write!(writer, "-1").expect("Failed to write");
        return;
    }

    // (ans - k) * n >= ans
    // ans * (n - 1) >= k * n
    // ans >= k * n / (n - 1)
    let ans = (k * n + (n - 2)) / (n - 1); // 올림 효과를 위해 n-2를 더하여 정수 나눗셈 처리

    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/23972
// 악마의 제안
#[test]
fn test_solve23972() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "24 2".to_string(),
            want: "48".to_string(),
        },
        TestData {
            s: "36 3".to_string(),
            want: "54".to_string(),
        },
        TestData {
            s: "100 1".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "100 2".to_string(),
            want: "200".to_string(),
        },
        TestData {
            s: "100 3".to_string(),
            want: "150".to_string(),
        },
        TestData {
            s: "5 3".to_string(),
            want: "8".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23972(&mut reader, &mut writer);

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
