use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18005(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_value(read_line(reader));

    // n이 짝수
    // - n % 4 == 0  → 홀 짝 홀 짝 | 짝 홀 짝 홀 → 짝 (홀 2개씩 합쳐 짝수)
    // - n % 4 != 0  → 홀 짝 | 짝 홀 → 홀 (홀 하나가 남아 홀수)
    //
    // n이 홀수
    // - (...): 짝수 개 묶음 = 홀 | 짝 가능
    // - (...) + 홀 | 짝 → 홀수 개는 이와 같이 표현 가능
    // - 홀 | 짝 + 홀 | 짝
    //   - 홀 + 홀 → 짝
    //   - 홀 + 짝 → 홀
    //   - 짝 + 홀 → 홀
    //   - 짝 + 짝 → 짝
    // - 따라서 홀 | 짝 모두 가능
    let ans = match n % 4 {
        1 | 3 => 0,
        0 => 2,
        _ => 1,
    };
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/18005
// Even or Odd?
#[test]
fn test_solve18005() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "12".to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18005(&mut reader, &mut writer);

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
