use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18312(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, k) = read_values_as!(read_line(reader), usize, usize);

    let mut ans = 0;
    for h in 0..=n {
        for m in 0..60 {
            for s in 0..60 {
                if format!("{:02}{:02}{:02}", h, m, s).contains(&format!("{}", k)) {
                    ans += 1;
                }
            }
        }
    }

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/18312
// 시각
#[test]
fn test_solve18312() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 3".to_string(),
            want: "11475".to_string(),
        },
        TestData {
            s: "0 0".to_string(),
            want: "3600".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18312(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
