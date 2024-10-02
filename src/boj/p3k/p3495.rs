use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3495(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (h, w) = read_values_as!(read_line(reader), usize, usize);

    let mut cnt = 0;
    for _ in 0..h {
        let s = read_line(reader);
        let iter = s.chars().take(w);
        let mut curr_cnt = 0;

        for c in iter {
            if c == '/' || c == '\\' {
                curr_cnt += 1;
                cnt += 1;
            } else if curr_cnt % 2 == 1 {
                cnt += 2;
            }
        }
    }

    write!(writer, "{}", cnt / 2).unwrap();
}

// https://www.acmicpc.net/problem/3495
// 아스키 도형
#[test]
fn test_solve3495() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4 4
/\\/\\
\\../
.\\.\\
..\\/"
                .to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "4 4
/\\/\\
\\../
/..\\
\\/\\/"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3495(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
