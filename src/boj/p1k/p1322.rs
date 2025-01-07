use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1322(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (x, k) = read_values_as!(read_line(reader), u64, u64);

    let mut result = 0u64;
    let mut k_pos = 0;
    let mut pos = 0;

    // k의 남은 비트를 모두 소진할 때까지
    while k_pos < 64 && (k >> k_pos) > 0 {
        // x의 해당 비트가 0이면 -> k의 해당 비트를 result에 삽입
        if x & (1 << pos) == 0 {
            if (k >> k_pos) & 1 == 1 {
                result |= 1 << pos;
            }
            k_pos += 1;
        }
        pos += 1;
    }

    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/1322
// X와 K
#[test]
fn test_solve1322() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 1".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5 5".to_string(),
            want: "18".to_string(),
        },
        TestData {
            s: "10 3".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "1 1000000000".to_string(),
            want: "2000000000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1322(&mut reader, &mut writer);

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
