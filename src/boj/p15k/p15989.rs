use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15989(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_value::<usize>(read_line(reader));

    // dp[i] = 1,2,3을 사용해 i를 만드는 (중복 순서 제외) 조합의 수
    let mut dp = vec![0_usize; 10001];

    dp[0] = 1; // "아무것도 선택 안 함"을 1가지로 본다.

    // coin=1부터, 차례대로 2, 3까지 누적
    for &coin in &[1, 2, 3] {
        for i in coin..=10000 {
            dp[i] += dp[i - coin];
        }
    }

    for _ in 0..t {
        let n = read_value::<usize>(read_line(reader));
        writeln!(writer, "{}", dp[n]).unwrap();
    }
}

// https://www.acmicpc.net/problem/15989
// 1, 2, 3 더하기 4
#[test]
fn test_solve15989() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
4
7
10"
            .to_string(),
            want: "4
8
14"
            .to_string(),
        },
        TestData {
            s: "10
10
20
40
80
160
320
640
1280
2560
5120"
                .to_string(),
            want: "14
44
154
574
2214
8694
34454
137174
547414
2187094"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15989(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("valid utf8 string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
