use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2875(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m, k) = read_values_as!(read_line(reader), usize, usize, usize);

    let (mut n, mut m, mut k) = (n, m, k);

    while k > 0 {
        if n >= 2 * m {
            n -= 1;
        } else {
            m -= 1;
        }
        k -= 1;
    }

    let ans = (n / 2).min(m);
    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/2875
// 대회 or 인턴
#[test]
fn test_solve2875() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6 3 2".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "8 3 5".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "100 0 100".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 1 0".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2 0 0".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 1 1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2 1 0".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2 2 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "2 1 1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6 10 3".to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2875(&mut reader, &mut writer);

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
